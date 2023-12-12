pub(crate) type Ind = (usize, usize);

/// Trait to provide abstraction over 2-dimensional vectors allowing access using indices.
///
/// Such an abstraction is particularly important in performance-critical algorithms both requiring flexibility through abstraction
/// over inputs and performance through monomorphization.
///
/// # Example
///
/// ```rust
/// use orx_funvec::*;
/// use orx_closure::Capture;
/// use std::collections::{BTreeMap, HashMap};
///
/// fn distance_between<V: FunVecD2<u32>>(distances: &V, a: usize, b: usize) -> Option<u32> {
///     distances.at(a, b)
/// }
///
/// // complete matrix
/// let jagged_vecs = vec![
///     vec![0, 1, 2, 3],
///     vec![10, 11, 12, 13],
///     vec![20, 21, 22, 23],
///     vec![30, 31, 32, 33],
/// ];
/// assert_eq!(Some(23), distance_between(&jagged_vecs, 2, 3));
/// assert_eq!(None, distance_between(&jagged_vecs, 2, 4));
/// assert_eq!(None, distance_between(&jagged_vecs, 4, 0));
///
/// // some sparsity in the first or second dimensions
/// let vec_of_maps = vec![
///     BTreeMap::from_iter([(1, 1), (14, 2)].into_iter()),
///     BTreeMap::from_iter([(0, 10), (7, 20)].into_iter()),
///     BTreeMap::from_iter([(9, 100), (16, 200)].into_iter()),
/// ];
/// assert_eq!(Some(20), distance_between(&vec_of_maps, 1, 7));
/// assert_eq!(None, distance_between(&vec_of_maps, 0, 0));
/// assert_eq!(None, distance_between(&vec_of_maps, 3, 0));
///
/// let map_of_vecs = HashMap::from_iter([
///     (1, vec![3, 4, 5]),
///     (7, vec![30, 40, 50]),
/// ].into_iter());
/// assert_eq!(Some(5), distance_between(&map_of_vecs, 1, 2));
/// assert_eq!(Some(40), distance_between(&map_of_vecs, 7, 1));
/// assert_eq!(None, distance_between(&map_of_vecs, 0, 0));
///
/// // complete sparsity
/// let map_of_indices = HashMap::from_iter([
///     ((0, 1), 14),
///     ((3, 6), 42),
/// ].into_iter());
/// assert_eq!(Some(14), distance_between(&map_of_indices, 0, 1));
/// assert_eq!(Some(42), distance_between(&map_of_indices, 3, 6));
/// assert_eq!(None, distance_between(&map_of_indices, 0, 0));
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
/// assert_eq!(Some(0), distance_between(&closure, 1, 1));
/// assert_eq!(Some(5), distance_between(&closure, 0, 1));
/// assert_eq!(None, distance_between(&closure, 0, 4));
///
/// // uniform distance for all pairs
/// let uniform = ScalarAsVec(42);
/// assert_eq!(Some(42), distance_between(&uniform, 7, 42));
///
/// // all disconnected pairs
/// let disconnected = EmptyVec::new();
/// assert_eq!(None, distance_between(&disconnected, 7, 42));
/// ```
pub trait FunVecD2<T> {
    type Iter<'a, I>: Iterator<Item = Option<T>> + 'a
    where
        T: 'a,
        Self: 'a,
        I: Iterator<Item = Ind> + 'a;

    /// Returns the value at the given `indices` or `None` if the position is empty.
    ///
    /// This allows to access elements of all funvec implementations in a unified way. Thanks to monomorphization, this abstraction does not have a performance penalty.
    ///
    /// Note that funvec's are different than, generalization of, traditional since the elements are not necessarily contagious or dense.
    /// Instead they can be sparse to desired degrees.
    ///
    /// Therefore, `at` always returns an optional.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_funvec::*;
    /// use orx_closure::Capture;
    /// use std::collections::{BTreeMap, HashMap};
    ///
    /// fn distance<V: FunVecD2<u32>>(distances: &V, a: usize, b: usize) -> Option<u32> {
    ///     distances.at(a, b)
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
    fn at(&self, i: usize, j: usize) -> Option<T>;

    /// Returns an iterator of elements of the vector for the given `indices`.
    ///
    /// `indices` can be any `Iterator` yielding `(usize, usize)` indices.
    ///
    /// This allows to iterate over all funvec implementations in a unified way. Thanks to monomorphization, this abstraction does not have a performance penalty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_funvec::*;
    /// use orx_closure::Capture;
    /// use std::collections::{BTreeMap, HashMap};
    ///
    /// fn total_distance<V, I>(distances: &V, pairs: I) -> u32
    /// where
    ///     V: FunVecD2<u32>,
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
    fn iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = Ind> + 'a;
}

/// Trait to provide abstraction over 2-dimensional vectors allowing reference access using indices.
///
/// Such an abstraction is particularly important in performance-critical algorithms both requiring flexibility through abstraction
/// over inputs and performance through monomorphization.
///
/// # Example
///
/// ```rust
/// use orx_funvec::*;
/// use orx_closure::Capture;
/// use std::collections::{BTreeMap, HashMap};
///
/// fn distance_between<V: FunVecRefD2<u32>>(distances: &V, a: usize, b: usize) -> Option<&u32> {
///     distances.ref_at(a, b)
/// }
///
/// // complete matrix
/// let jagged_vecs = vec![
///     vec![0, 1, 2, 3],
///     vec![10, 11, 12, 13],
///     vec![20, 21, 22, 23],
///     vec![30, 31, 32, 33],
/// ];
/// assert_eq!(Some(&23), distance_between(&jagged_vecs, 2, 3));
/// assert_eq!(None, distance_between(&jagged_vecs, 2, 4));
/// assert_eq!(None, distance_between(&jagged_vecs, 4, 0));
///
/// // some sparsity in the first or second dimensions
/// let vec_of_maps = vec![
///     BTreeMap::from_iter([(1, 1), (14, 2)].into_iter()),
///     BTreeMap::from_iter([(0, 10), (7, 20)].into_iter()),
///     BTreeMap::from_iter([(9, 100), (16, 200)].into_iter()),
/// ];
/// assert_eq!(Some(&20), distance_between(&vec_of_maps, 1, 7));
/// assert_eq!(None, distance_between(&vec_of_maps, 0, 0));
/// assert_eq!(None, distance_between(&vec_of_maps, 3, 0));
///
/// let map_of_vecs = HashMap::from_iter([
///     (1, vec![3, 4, 5]),
///     (7, vec![30, 40, 50]),
/// ].into_iter());
/// assert_eq!(Some(&5), distance_between(&map_of_vecs, 1, 2));
/// assert_eq!(Some(&40), distance_between(&map_of_vecs, 7, 1));
/// assert_eq!(None, distance_between(&map_of_vecs, 0, 0));
///
/// // complete sparsity
/// let map_of_indices = HashMap::from_iter([
///     ((0, 1), 14),
///     ((3, 6), 42),
/// ].into_iter());
/// assert_eq!(Some(&14), distance_between(&map_of_indices, 0, 1));
/// assert_eq!(Some(&42), distance_between(&map_of_indices, 3, 6));
/// assert_eq!(None, distance_between(&map_of_indices, 0, 0));
///
/// // uniform distance for all pairs
/// let uniform = ScalarAsVec(42);
/// assert_eq!(Some(&42), distance_between(&uniform, 7, 42));
///
/// // all disconnected pairs
/// let disconnected = EmptyVec::new();
/// assert_eq!(None, distance_between(&disconnected, 7, 42));
/// ```
pub trait FunVecRefD2<T: ?Sized> {
    type Iter<'a, I>: Iterator<Item = Option<&'a T>> + 'a
    where
        T: 'a,
        Self: 'a,
        I: Iterator<Item = Ind> + 'a;

    /// Returns a reference to the element at the given `indices` or `None` if the position is empty.
    ///
    /// This allows to access elements of all funvec implementations in a unified way. Thanks to monomorphization, this abstraction does not have a performance penalty.
    ///
    /// Note that funvec's are different than, generalization of, traditional since the elements are not necessarily contagious or dense.
    /// Instead they can be sparse to desired degrees.
    ///
    /// Therefore, `ref_at` always returns an optional.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_funvec::*;
    /// use orx_closure::Capture;
    /// use std::collections::{BTreeMap, HashMap};
    ///
    /// fn distance<V: FunVecRefD2<u32>>(distances: &V, a: usize, b: usize) -> Option<&u32> {
    ///     distances.ref_at(a, b)
    /// }
    ///
    /// // complete matrix
    /// let jagged_vecs = vec![
    ///     vec![0, 1, 2, 3],
    ///     vec![10, 11, 12, 13],
    ///     vec![20, 21, 22, 23],
    ///     vec![30, 31, 32, 33],
    /// ];
    /// assert_eq!(Some(&23), distance(&jagged_vecs, 2, 3));
    /// assert_eq!(None, distance(&jagged_vecs, 2, 4));
    /// assert_eq!(None, distance(&jagged_vecs, 4, 0));
    ///
    /// // some sparsity in the first or second dimensions
    /// let vec_of_maps = vec![
    ///     BTreeMap::from_iter([(1, 1), (14, 2)].into_iter()),
    ///     BTreeMap::from_iter([(0, 10), (7, 20)].into_iter()),
    ///     BTreeMap::from_iter([(9, 100), (16, 200)].into_iter()),
    /// ];
    /// assert_eq!(Some(&20), distance(&vec_of_maps, 1, 7));
    /// assert_eq!(None, distance(&vec_of_maps, 0, 0));
    /// assert_eq!(None, distance(&vec_of_maps, 3, 0));
    ///
    /// let map_of_vecs = HashMap::from_iter([
    ///     (1, vec![3, 4, 5]),
    ///     (7, vec![30, 40, 50]),
    /// ].into_iter());
    /// assert_eq!(Some(&5), distance(&map_of_vecs, 1, 2));
    /// assert_eq!(Some(&40), distance(&map_of_vecs, 7, 1));
    /// assert_eq!(None, distance(&map_of_vecs, 0, 0));
    ///
    /// // complete sparsity
    /// let map_of_indices = HashMap::from_iter([
    ///     ((0, 1), 14),
    ///     ((3, 6), 42),
    /// ].into_iter());
    /// assert_eq!(Some(&14), distance(&map_of_indices, 0, 1));
    /// assert_eq!(Some(&42), distance(&map_of_indices, 3, 6));
    /// assert_eq!(None, distance(&map_of_indices, 0, 0));
    ///
    /// // uniform distance for all pairs
    /// let uniform = ScalarAsVec(42);
    /// assert_eq!(Some(&42), distance(&uniform, 7, 42));
    ///
    /// // all disconnected pairs
    /// let disconnected = EmptyVec::new();
    /// assert_eq!(None, distance(&disconnected, 7, 42));
    /// ```
    fn ref_at(&self, i: usize, j: usize) -> Option<&T>;

    /// Returns an iterator yielding references to elements of the vector for the given `indices`.
    ///
    /// `indices` can be any `Iterator` yielding `(usize, usize)` indices.
    ///
    /// This allows to iterate over all funvec implementations in a unified way. Thanks to monomorphization, this abstraction does not have a performance penalty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_funvec::*;
    /// use orx_closure::Capture;
    /// use std::collections::{BTreeMap, HashMap};
    ///
    /// fn total_distance<V, I>(distances: &V, pairs: I) -> u32
    /// where
    ///     V: FunVecRefD2<u32>,
    ///     I: Iterator<Item = (usize, usize)>
    /// {
    ///     distances.ref_iter_over(pairs).flatten().sum()
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
    fn ref_iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = Ind> + 'a;
}
