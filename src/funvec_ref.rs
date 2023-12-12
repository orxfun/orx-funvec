use crate::{index::IntoIndex, iter_over_ref::IterOverRefs};

/// Trait to provide abstraction over `DIM`-dimensional vectors allowing reference access using indices.
///
/// Such an abstraction is particularly important in performance-critical algorithms both requiring flexibility through abstraction
/// over inputs and performance through monomorphization.
///
/// This trait for a given or generic `DIM` can be extended by implementing `fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T>`.
///
/// # Examples - Dimension 1
///
/// ```rust
/// use orx_funvec::*;
///
/// fn moving_average<V: FunVecRef<1, i32>>(observations: &V, period: usize) -> Option<i32> {
///     let last = if period == 0 { None } else { observations.ref_at(period - 1) };
///     let current = observations.ref_at(period);
///
///     match (last, current) {
///         (None, None) => None,
///         (None, Some(y)) => Some(*y),
///         (Some(x), None) => Some(*x),
///         (Some(x), Some(y)) => Some((x + y) / 2)
///     }
/// }
///
/// let period = 2;
///
/// let stdvec = vec![10, 11, 12, 13];
/// assert_eq!(Some(11), moving_average(&stdvec, period));
///
/// let map = std::collections::HashMap::from_iter([(1, 10), (2, 20), (3, 30)].into_iter());
/// assert_eq!(Some(15), moving_average(&map, period));
///
/// let closure = orx_closure::Capture(())
///     .fun_option_ref(|_, i: usize| Some(if i == 2 { &20 } else { &30 }));
/// assert_eq!(Some(25), moving_average(&closure, period));
///
/// let uniform = ScalarAsVec(42);
/// assert_eq!(Some(42), moving_average(&uniform, period));
///
/// let no_data = EmptyVec::new();
/// assert_eq!(None, moving_average(&no_data, period));
/// ```
///
/// # Examples - Dimension 2
///
/// ```rust
/// use orx_funvec::*;
/// use orx_closure::Capture;
/// use std::collections::{BTreeMap, HashMap};
///
/// fn distance_between<V: FunVecRef<2, u32>>(distances: &V, a: usize, b: usize) -> Option<&u32> {
///     distances.ref_at((a, b))
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
pub trait FunVecRef<const DIM: usize, T>
where
    T: ?Sized,
{
    /// Returns a reference to the element at the given `index` or `None` if the position is empty.
    ///
    /// This allows to access elements of all funvec implementations in a unified way. Thanks to monomorphization, this abstraction does not have a performance penalty.
    ///
    /// Note that funvec's are different than, generalization of, traditional vectors since the elements are not necessarily contagious or dense.
    /// Instead they can be sparse to desired degrees.
    ///
    /// Therefore, `ref_at` always returns an optional.
    ///
    /// # Examples - Dimension 1
    ///
    /// ```rust
    /// use orx_funvec::*;
    /// use orx_closure::Capture;
    /// use std::collections::{HashSet, HashMap};
    ///
    /// struct Student {
    ///     name: String,
    ///     age: u32,
    /// }
    /// impl Student {
    ///     fn new(name: &str, age: u32) -> Self {
    ///         Self { name: name.to_string(), age }
    ///     }
    /// }
    ///
    /// fn average_student_age<S: FunVecRef<1, Student>>(students: &S, ids: &HashSet<usize>) -> u32 {
    ///     let mut sum_age = 0;
    ///     let mut num_students = 0;
    ///     for id in ids {
    ///         if let Some(student) = students.ref_at(*id) {
    ///             sum_age += student.age;
    ///             num_students += 1;
    ///         }
    ///     }
    ///     if num_students == 0 {
    ///         0
    ///     } else {
    ///         sum_age / num_students
    ///     }
    /// }
    ///
    /// let ids = HashSet::from_iter([0, 2, 3].into_iter());
    ///
    /// let stdvec = vec![Student::new("foo", 18), Student::new("bar", 22), Student::new("baz", 16)];
    /// assert_eq!(17, average_student_age(&stdvec, &ids));
    ///
    /// let map = HashMap::from_iter([
    ///     (0, Student::new("foo", 18)),
    ///     (3, Student::new("bar", 20)),
    ///     (10, Student::new("baz", 30)),
    /// ].into_iter());
    /// assert_eq!(19, average_student_age(&map, &ids));
    ///
    /// let regular = vec![Student::new("foo", 18), Student::new("bar", 22), Student::new("baz", 16)];
    /// let exchange: HashMap<usize, Student> = HashMap::from_iter([
    ///     (3, Student::new("john", 20)),
    ///     (42, Student::new("doe", 17)),
    /// ].into_iter());
    /// let closure = Capture((regular, exchange)).fun_option_ref(|(r, x), i: usize| {
    ///     r.get(i).or(x.get(&i))
    /// });
    /// assert_eq!(18, average_student_age(&closure, &ids));
    ///
    /// let only_foo = ScalarAsVec(Student::new("foo", 42));
    /// assert_eq!(42, average_student_age(&only_foo, &ids));
    ///
    /// let no_students = EmptyVec::new();
    /// assert_eq!(0, average_student_age(&no_students, &ids));
    /// ```
    ///
    /// # Examples - Dimension 2
    ///
    /// ```rust
    /// use orx_funvec::*;
    /// use orx_closure::Capture;
    /// use std::collections::{BTreeMap, HashMap};
    ///
    /// fn distance<V: FunVecRef<2, u32>>(distances: &V, a: usize, b: usize) -> Option<&u32> {
    ///     distances.ref_at([a, b])
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
    fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T>;

    /// Returns an iterator yielding references to elements of the vector for the given `indices`.
    ///
    /// `indices` can be any `Iterator` yielding `Idx` indices, where `Idx` can be any primitive that can be converted into `[usize; DIM]`.
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
    /// use std::collections::{HashSet, HashMap};
    ///
    /// struct Student {
    ///     name: String,
    ///     age: u32,
    /// }
    /// impl Student {
    ///     fn new(name: &str, age: u32) -> Self {
    ///         Self { name: name.to_string(), age }
    ///     }
    /// }
    ///
    /// fn average_student_age<S: FunVecRef<1, Student>>(students: &S, ids: &HashSet<usize>) -> u32 {
    ///     let (count, sum) = students.ref_iter_over(ids.iter().copied())
    ///         .flatten()
    ///         .map(|student| student.age)
    ///         .fold((0, 0), |(count, sum), value| (count + 1, sum + value));
    ///
    ///     if count == 0 { 0 } else { sum / count }
    /// }
    ///
    /// let ids = HashSet::from_iter([0, 2, 3].into_iter());
    ///
    /// let stdvec = vec![Student::new("foo", 18), Student::new("bar", 22), Student::new("baz", 16)];
    /// assert_eq!(17, average_student_age(&stdvec, &ids));
    ///
    /// let map = HashMap::from_iter([
    ///     (0, Student::new("foo", 18)),
    ///     (3, Student::new("bar", 20)),
    ///     (10, Student::new("baz", 30)),
    /// ].into_iter());
    /// assert_eq!(19, average_student_age(&map, &ids));
    ///
    /// let regular = vec![Student::new("foo", 18), Student::new("bar", 22), Student::new("baz", 16)];
    /// let exchange: HashMap<usize, Student> = HashMap::from_iter([
    ///     (3, Student::new("john", 20)),
    ///     (42, Student::new("doe", 17)),
    /// ].into_iter());
    /// let closure = Capture((regular, exchange))
    ///     .fun_option_ref(|(r, x), i: usize| r.get(i).or(x.get(&i)));
    /// assert_eq!(18, average_student_age(&closure, &ids));
    ///
    /// let only_foo = ScalarAsVec(Student::new("foo", 42));
    /// assert_eq!(42, average_student_age(&only_foo, &ids));
    ///
    /// let no_students = EmptyVec::new();
    /// assert_eq!(0, average_student_age(&no_students, &ids));
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
    ///     V: FunVecRef<2, u32>,
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
    fn ref_iter_over<'a, Idx, IdxIter>(
        &self,
        indices: IdxIter,
    ) -> IterOverRefs<DIM, T, Idx, IdxIter, Self>
    where
        Idx: IntoIndex<DIM>,
        IdxIter: Iterator<Item = Idx> + 'a,
    {
        IterOverRefs::new(self, indices)
    }
}
