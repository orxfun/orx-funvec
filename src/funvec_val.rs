use crate::{index::IntoIndex, iter_over_val::IterOverValues};

/// Trait to provide abstraction over `DIM`-dimensional vectors allowing access using indices.
///
/// Such an abstraction is particularly important in performance-critical algorithms both requiring flexibility through abstraction
/// over inputs and performance through monomorphization.
///
/// This trait for a given or generic `DIM` can be extended by implementing `fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T>`.
///
/// # Examples
///
/// ## Dimension 1 Example
///
/// ```rust
/// use orx_funvec::*;
/// use orx_closure::Capture;
/// use std::collections::HashMap;
///
/// fn moving_average<V: FunVec<1, i32>>(observations: &V, period: usize) -> Option<i32> {
///     let last = if period == 0 { None } else { observations.at(period - 1) };
///     let current = observations.at(period);
///
///     match (last, current) {
///         (None, None) => None,
///         (None, Some(y)) => Some(y),
///         (Some(x), None) => Some(x),
///         (Some(x), Some(y)) => Some((x + y) / 2)
///     }
/// }
///
/// let period = 2;
///
/// let stdvec = vec![10, 11, 12, 13];
/// assert_eq!(Some(11), moving_average(&stdvec, period));
///
/// let map = HashMap::from_iter([(1, 10), (2, 20), (3, 30)].into_iter());
/// assert_eq!(Some(15), moving_average(&map, period));
///
/// let closure = Capture(()).fun(|_, i: usize| Some(if i == 2 { 20 } else { 30 }));
/// assert_eq!(Some(25), moving_average(&closure, period));
///
/// let uniform = ScalarAsVec(42);
/// assert_eq!(Some(42), moving_average(&uniform, period));
///
/// let no_data = EmptyVec::new();
/// assert_eq!(None, moving_average(&no_data, period));
/// ```
///
/// ## Dimension 2 Example
///
/// ```rust
/// use orx_funvec::*;
/// use orx_closure::Capture;
/// use std::collections::{BTreeMap, HashMap};
///
/// fn distance<V: FunVec<2, u32>>(distances: &V, a: usize, b: usize) -> Option<u32> {
///     distances.at([a, b])
/// }
///
/// // complete matrix
/// let jagged_vecs = vec![
///     vec![0, 1, 2, 3],
///     vec![10, 11, 12, 13],
///     vec![20, 21, 22, 23],
///     vec![30, 31, 32, 33],
/// ];
/// assert_eq!(Some(23), distance(&jagged_vecs, 2, 3));
/// assert_eq!(None, distance(&jagged_vecs, 2, 4));
/// assert_eq!(None, distance(&jagged_vecs, 4, 0));
///
/// // some sparsity in the first or second dimensions
/// let vec_of_maps = vec![
///     BTreeMap::from_iter([(1, 1), (14, 2)].into_iter()),
///     BTreeMap::from_iter([(0, 10), (7, 20)].into_iter()),
///     BTreeMap::from_iter([(9, 100), (16, 200)].into_iter()),
/// ];
/// assert_eq!(Some(20), distance(&vec_of_maps, 1, 7));
/// assert_eq!(None, distance(&vec_of_maps, 0, 0));
/// assert_eq!(None, distance(&vec_of_maps, 3, 0));
///
/// let map_of_vecs = HashMap::from_iter([
///     (1, vec![3, 4, 5]),
///     (7, vec![30, 40, 50]),
/// ].into_iter());
/// assert_eq!(Some(5), distance(&map_of_vecs, 1, 2));
/// assert_eq!(Some(40), distance(&map_of_vecs, 7, 1));
/// assert_eq!(None, distance(&map_of_vecs, 0, 0));
///
/// // complete sparsity
/// let map_of_indices = HashMap::from_iter([
///     ((0, 1), 14),
///     ((3, 6), 42),
/// ].into_iter());
/// assert_eq!(Some(14), distance(&map_of_indices, 0, 1));
/// assert_eq!(Some(42), distance(&map_of_indices, 3, 6));
/// assert_eq!(None, distance(&map_of_indices, 0, 0));
///
/// // closure to compute distances on the fly rather than to store them
/// fn get_euclidean_distance(location1: (f64, f64), location2: (f64, f64)) -> u32 {
///     let (x1, y1) = location1;
///     let (x2, y2) = location2;
///     (f64::powf(x1 - x2, 2.0) + f64::powf(y1 - y2, 2.0)).sqrt() as u32
/// }
/// let locations = vec![(0.0, 1.0), (3.0, 5.0), (7.0, 2.0), (1.0, 1.0)];
/// let closure = Capture(&locations).fun(|loc, (i, j): (usize, usize)| {
///     loc.get(i)
///         .and_then(|l1| loc.get(j).map(|l2| (l1, l2)))
///         .map(|(l1, l2)| get_euclidean_distance(*l1, *l2))
/// });
/// assert_eq!(Some(0), distance(&closure, 1, 1));
/// assert_eq!(Some(5), distance(&closure, 0, 1));
/// assert_eq!(None, distance(&closure, 0, 4));
///
/// // uniform distance for all pairs
/// let uniform = ScalarAsVec(42);
/// assert_eq!(Some(42), distance(&uniform, 7, 42));
///
/// // all disconnected pairs
/// let disconnected = EmptyVec::new();
/// assert_eq!(None, distance(&disconnected, 7, 42));
/// ```
///
/// ## Extension
///
/// Implementing the trait requires implementation of only the `fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T>` method.
///
/// Assume we are working with distance matrices.
/// In certain scenarios, we observe that we access only a limited number of pairs.
/// Assuming the distance computation is expensive, we do not want to populate and store the entire matrix.
/// Instead, we implement a distance provider with caching capabilities.
/// The goal is to be able to use this provider as a generic distance matrix, and hence, we implement `FunVec<2, _>`.
///
/// ```rust
/// use orx_funvec::*;
/// use std::{cell::RefCell, collections::HashMap};
///
/// type Dist = u32;
///
/// struct DistanceProvider {
///     locations: Vec<(i32, i32)>,
///     cached: RefCell<HashMap<(usize, usize), Dist>>,
/// }
/// impl DistanceProvider {
///     fn distance(&self, from: usize, to: usize) -> Option<Dist> {
///         if let Some(cached) = self.cached.borrow().get(&(from, to)) {
///             return Some(*cached);
///         }
///         let locations = self
///             .locations
///             .get(from)
///             .and_then(|l1| self.locations.get(to).map(|l2| (l1, l2)));
///
///         if let Some((l1, l2)) = locations {
///             let (x1, y1) = l1;
///             let (x2, y2) = l2;
///             let distance =
///                 ((i32::pow(x1 - x2, 2) + i32::pow(y1 - y2, 2)) as f32).sqrt() as Dist;
///
///             // cache computed distance
///             self.cached.borrow_mut().insert((from, to), distance);
///
///             Some(distance)
///         } else {
///             None
///         }
///     }
/// }
///
/// impl FunVec<2, Dist> for DistanceProvider {
///     fn at<Idx: IntoIndex<2>>(&self, index: Idx) -> Option<Dist> {
///         let [from, to] = index.into_index();
///         self.distance(from, to)
///     }
/// }
///
/// let locations = vec![(0, 1), (3, 5), (7, 2), (1, 2)];
/// let distances = DistanceProvider {
///     locations,
///     cached: RefCell::new(HashMap::new()),
/// };
///
/// assert_eq!(Some(5), distances.at([0, 1]));
/// assert_eq!(Some(5), distances.at([0, 1])); // from cache
/// assert_eq!(None, distances.at([0, 4]));
///
/// let pairs = vec![(0, 1), (2, 3)];
/// assert_eq!(
///     11u32,
///     distances.iter_over(pairs.iter().copied()).flatten().sum()
/// );
/// ```
pub trait FunVec<const DIM: usize, T>
where
    T: Clone + Copy,
{
    /// Returns the value at the given `index` or `None` if the position is empty.
    ///
    /// This allows to access elements of all funvec implementations in a unified way. Thanks to monomorphization, this abstraction does not have a performance penalty.
    ///
    /// Note that funvec's are different than, generalization of, traditional vectors since the elements are not necessarily contagious or dense.
    /// Instead they can be sparse to desired degrees.
    ///
    /// Therefore, `at` always returns an optional.
    ///
    /// # Examples - Dimension 1
    ///
    /// ```rust
    /// use orx_funvec::*;
    /// use orx_closure::Capture;
    /// use std::collections::HashMap;
    ///
    /// let stdvec = vec![10, 11, 12, 13];
    /// assert_eq!(Some(13), stdvec.at(3));
    /// assert_eq!(None, stdvec.at(4));
    ///
    /// let map = HashMap::from_iter([(1, 10), (2, 20)].into_iter());
    /// assert_eq!(Some(10), map.at(1));
    /// assert_eq!(None, map.at(0));
    ///
    /// let (s, t) = (0, 42);
    /// let closure = Capture((s, t))
    ///     .fun(|(s, t), i: usize| if i == *s { Some(1) } else if i == *t { Some(-1) } else { None });
    /// assert_eq!(Some(1), closure.at(0));
    /// assert_eq!(Some(-1), closure.at(42));
    /// assert_eq!(None, closure.at(3));
    ///
    /// let scalar = ScalarAsVec(42);
    /// assert_eq!(Some(42), scalar.at(7));
    /// assert_eq!(Some(42), scalar.at(12));
    ///
    /// let empty_vec: EmptyVec<i32> = EmptyVec::new();
    /// assert_eq!(None, empty_vec.at([7]));
    /// assert_eq!(None, empty_vec.at([12]));
    /// ```
    ///
    /// # Examples - Dimension 2
    ///
    /// ```rust
    /// use orx_funvec::*;
    /// use orx_closure::Capture;
    /// use std::collections::{BTreeMap, HashMap};
    ///
    /// fn distance<V: FunVec<2, u32>>(distances: &V, a: usize, b: usize) -> Option<u32> {
    ///     distances.at([a, b])
    /// }
    ///
    /// // complete matrix
    /// let jagged_vecs = vec![
    ///     vec![0, 1, 2, 3],
    ///     vec![10, 11, 12, 13],
    ///     vec![20, 21, 22, 23],
    ///     vec![30, 31, 32, 33],
    /// ];
    /// assert_eq!(Some(23), distance(&jagged_vecs, 2, 3));
    /// assert_eq!(None, distance(&jagged_vecs, 2, 4));
    /// assert_eq!(None, distance(&jagged_vecs, 4, 0));
    ///
    /// // some sparsity in the first or second dimensions
    /// let vec_of_maps = vec![
    ///     BTreeMap::from_iter([(1, 1), (14, 2)].into_iter()),
    ///     BTreeMap::from_iter([(0, 10), (7, 20)].into_iter()),
    ///     BTreeMap::from_iter([(9, 100), (16, 200)].into_iter()),
    /// ];
    /// assert_eq!(Some(20), distance(&vec_of_maps, 1, 7));
    /// assert_eq!(None, distance(&vec_of_maps, 0, 0));
    /// assert_eq!(None, distance(&vec_of_maps, 3, 0));
    ///
    /// let map_of_vecs = HashMap::from_iter([
    ///     (1, vec![3, 4, 5]),
    ///     (7, vec![30, 40, 50]),
    /// ].into_iter());
    /// assert_eq!(Some(5), distance(&map_of_vecs, 1, 2));
    /// assert_eq!(Some(40), distance(&map_of_vecs, 7, 1));
    /// assert_eq!(None, distance(&map_of_vecs, 0, 0));
    ///
    /// // complete sparsity
    /// let map_of_indices = HashMap::from_iter([
    ///     ((0, 1), 14),
    ///     ((3, 6), 42),
    /// ].into_iter());
    /// assert_eq!(Some(14), distance(&map_of_indices, 0, 1));
    /// assert_eq!(Some(42), distance(&map_of_indices, 3, 6));
    /// assert_eq!(None, distance(&map_of_indices, 0, 0));
    ///
    /// // closure to compute distances on the fly rather than to store them
    /// fn get_euclidean_distance(location1: (f64, f64), location2: (f64, f64)) -> u32 {
    ///     let (x1, y1) = location1;
    ///     let (x2, y2) = location2;
    ///     (f64::powf(x1 - x2, 2.0) + f64::powf(y1 - y2, 2.0)).sqrt() as u32
    /// }
    /// let locations = vec![(0.0, 1.0), (3.0, 5.0), (7.0, 2.0), (1.0, 1.0)];
    /// let closure = Capture(&locations).fun(|loc, (i, j): (usize, usize)| {
    ///     loc.get(i)
    ///         .and_then(|l1| loc.get(j).map(|l2| (l1, l2)))
    ///         .map(|(l1, l2)| get_euclidean_distance(*l1, *l2))
    /// });
    /// assert_eq!(Some(0), distance(&closure, 1, 1));
    /// assert_eq!(Some(5), distance(&closure, 0, 1));
    /// assert_eq!(None, distance(&closure, 0, 4));
    ///
    /// // uniform distance for all pairs
    /// let uniform = ScalarAsVec(42);
    /// assert_eq!(Some(42), distance(&uniform, 7, 42));
    ///
    /// // all disconnected pairs
    /// let disconnected = EmptyVec::new();
    /// assert_eq!(None, distance(&disconnected, 7, 42));
    /// ```
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T>;

    /// Returns an iterator of elements of the vector for the given `indices`.
    ///
    /// `indices` can be any `Iterator` yielding `Idx` indices, where `Idx` can be any usize-primitive that can be converted into `[usize; DIM]`.
    /// For instance:
    /// * `usize` or `(usize)` can be converted into `[usize; 1]`,
    /// * `(usize, usize)` can be converted into `[usize; 2]`.
    ///
    /// This allows to iterate over all funvec implementations in a unified way. Thanks to monomorphization, this abstraction does not have a performance penalty.
    ///
    /// # Examples - Dimension 1
    ///
    /// ```rust
    /// use orx_funvec::*;
    /// use orx_closure::Capture;
    /// use std::collections::HashMap;
    ///
    /// fn sum_values<V: FunVec<1, i32>, I: Iterator<Item = usize>>(vec: &V, indices: I) -> i32 {
    ///     vec.iter_over(indices).flatten().sum()
    /// }
    ///
    /// let stdvec = vec![10, 11, 12, 13];
    /// assert_eq!(23, sum_values(&stdvec, 1..3));
    ///
    /// let map = HashMap::from_iter([(1, 10), (8, 20), (12, 200)].into_iter());
    /// assert_eq!(20, sum_values(&map, (1..10).filter(|x| x % 2 == 0)));
    ///
    /// let (s, t) = (0, 42);
    /// let closure = Capture((s, t))
    ///     .fun(|(s, t), i: usize| if i == *s { Some(10) } else if i == *t { Some(-1) } else { None });
    /// assert_eq!(9, sum_values(&closure, [0, 21, 42].iter().copied()));
    ///
    /// let scalar = ScalarAsVec(21);
    /// assert_eq!(42, sum_values(&scalar, 1..3));
    ///
    /// let empty_vec: EmptyVec<i32> = EmptyVec::new();
    /// assert_eq!(0, sum_values(&empty_vec, 1..3));
    /// ```
    ///
    /// # Examples - Dimension 2
    ///
    /// ```rust
    /// use orx_funvec::*;
    /// use orx_closure::Capture;
    /// use std::collections::{BTreeMap, HashMap};
    ///
    /// fn total_distance<V, I>(distances: &V, pairs: I) -> u32
    /// where
    ///     V: FunVec<2, u32>,
    ///     I: Iterator<Item = (usize, usize)>
    /// {
    ///     distances.iter_over(pairs).flatten().sum()
    /// }
    ///
    /// // complete matrix
    /// let jagged_vecs = vec![
    ///     vec![0, 1, 2, 3],
    ///     vec![10, 11, 12, 13],
    ///     vec![20, 21, 22, 23],
    ///     vec![30, 31, 32, 33],
    /// ];
    /// assert_eq!(0 + 1 + 10 + 11,
    ///     total_distance(&jagged_vecs, (0..2).flat_map(|i| (0..2).map(move |j| (i, j)))));
    /// assert_eq!(12 + 33, total_distance(&jagged_vecs, [(1, 2), (3, 3), (10, 10)].iter().copied()));
    ///
    /// // some sparsity in the first or second dimensions
    /// let vec_of_maps = vec![
    ///     BTreeMap::from_iter([(1, 1), (14, 2)].into_iter()),
    ///     BTreeMap::from_iter([(0, 10), (7, 20)].into_iter()),
    ///     BTreeMap::from_iter([(9, 100), (16, 200)].into_iter()),
    /// ];
    /// assert_eq!(1 + 10,
    ///     total_distance(&vec_of_maps, (0..2).flat_map(|i| (0..2).map(move |j| (i, j)))));
    /// assert_eq!(20 + 100,
    ///     total_distance(&vec_of_maps, [(1, 7), (2, 9)].iter().copied()));
    ///
    /// let map_of_vecs = HashMap::from_iter([
    ///     (1, vec![3, 4, 5]),
    ///     (7, vec![30, 40, 50]),
    /// ].into_iter());
    /// assert_eq!(3 + 4,
    ///     total_distance(&map_of_vecs, (0..2).flat_map(|i| (0..2).map(move |j| (i, j)))));
    /// assert_eq!(5 + 40,
    ///     total_distance(&map_of_vecs, [(1, 2), (7, 1)].iter().copied()));
    ///
    /// // complete sparsity
    /// let map_of_indices = HashMap::from_iter([
    ///     ((0, 1), 14),
    ///     ((3, 6), 42),
    /// ].into_iter());
    /// assert_eq!(14,
    ///     total_distance(&map_of_indices, (0..2).flat_map(|i| (0..2).map(move |j| (i, j)))));
    /// assert_eq!(14 + 42,
    ///     total_distance(&map_of_indices, [(0, 1), (3, 6), (100, 100)].iter().copied()));
    ///
    /// // closure to compute distances on the fly rather than to store them
    /// fn get_euclidean_distance(location1: (f64, f64), location2: (f64, f64)) -> u32 {
    ///     let (x1, y1) = location1;
    ///     let (x2, y2) = location2;
    ///     (f64::powf(x1 - x2, 2.0) + f64::powf(y1 - y2, 2.0)).sqrt() as u32
    /// }
    /// let locations = vec![(0.0, 1.0), (3.0, 5.0), (7.0, 2.0), (1.0, 1.0)];
    /// let closure = Capture(&locations).fun(|loc, (i, j): (usize, usize)| {
    ///     loc.get(i)
    ///         .and_then(|l1| loc.get(j).map(|l2| (l1, l2)))
    ///         .map(|(l1, l2)| get_euclidean_distance(*l1, *l2))
    /// });
    /// assert_eq!(2 * 5,
    ///     total_distance(&closure, (0..2).flat_map(|i| (0..2).map(move |j| (i, j)))));
    /// assert_eq!(5 + 1,
    ///     total_distance(&closure, [(0, 1), (3, 0), (100, 100)].iter().copied()));
    ///
    /// // uniform distance for all pairs
    /// let uniform = ScalarAsVec(42);
    /// assert_eq!(4 * 42,
    ///     total_distance(&uniform, (0..2).flat_map(|i| (0..2).map(move |j| (i, j)))));
    /// assert_eq!(42 * 3,
    ///     total_distance(&uniform, [(0, 1), (3, 0), (100, 100)].iter().copied()));
    ///
    /// // all disconnected pairs
    /// let disconnected = EmptyVec::new();
    /// assert_eq!(0,
    ///     total_distance(&disconnected, (0..2).flat_map(|i| (0..2).map(move |j| (i, j)))));
    /// assert_eq!(0,
    ///     total_distance(&disconnected, [(0, 1), (3, 0), (100, 100)].iter().copied()));
    /// ```
    fn iter_over<'a, Idx, IdxIter>(
        &self,
        indices: IdxIter,
    ) -> IterOverValues<DIM, T, Idx, IdxIter, Self>
    where
        Idx: IntoIndex<DIM>,
        IdxIter: Iterator<Item = Idx> + 'a,
    {
        IterOverValues::new(self, indices)
    }
}
