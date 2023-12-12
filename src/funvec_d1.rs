/// Trait to provide abstraction over 1-dimensional vectors allowing access using indices.
///
/// Such an abstraction is particularly important in performance-critical algorithms both requiring flexibility through abstraction
/// over inputs and performance through monomorphization.
///
/// # Example
///
/// ```rust
/// use orx_funvec::*;
///
/// fn moving_average<V: FunVecD1<i32>>(observations: &V, period: usize) -> Option<i32> {
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
/// let map = std::collections::HashMap::from_iter([(1, 10), (2, 20), (3, 30)].into_iter());
/// assert_eq!(Some(15), moving_average(&map, period));
///
/// let closure = orx_closure::Capture(())
///     .fun(|_, i: usize| Some(if i == 2 { 20 } else { 30 }));
/// assert_eq!(Some(25), moving_average(&closure, period));
///
/// let uniform = ScalarAsVec(42);
/// assert_eq!(Some(42), moving_average(&uniform, period));
///
/// let no_data = EmptyVec::new();
/// assert_eq!(None, moving_average(&no_data, period));
/// ```
pub trait FunVecD1<T> {
    type Iter<'a, I>: Iterator<Item = Option<T>> + 'a
    where
        T: 'a,
        Self: 'a,
        I: Iterator<Item = usize> + 'a;

    /// Returns the value at the given `index` or `None` if the position is empty.
    ///
    /// This allows to access elements of all funvec implementations in a unified way. Thanks to monomorphization, this abstraction does not have a performance penalty.
    ///
    /// Note that funvec's are different than, generalization of, traditional since the elements are not necessarily contagious:
    /// * there might exist values for indices 0 and 1;
    /// * positions 2 and 3 might be empty returning `None`;
    /// * and there might again be a value at position 4.
    ///
    /// Therefore, `at` always returns an optional.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_funvec::*;
    ///
    /// let stdvec = vec![10, 11, 12, 13];
    /// assert_eq!(Some(13), stdvec.at(3));
    /// assert_eq!(None, stdvec.at(4));
    ///
    /// let map = std::collections::HashMap::from_iter([(1, 10), (2, 20)].into_iter());
    /// assert_eq!(Some(10), map.at(1));
    /// assert_eq!(None, map.at(0));
    ///
    /// let (s, t) = (0, 42);
    /// let closure = orx_closure::Capture((s, t))
    ///     .fun(|(s, t), i: usize| if i == *s { Some(1) } else if i == *t { Some(-1) } else { None });
    /// assert_eq!(Some(1), closure.at(0));
    /// assert_eq!(Some(-1), closure.at(42));
    /// assert_eq!(None, closure.at(3));
    ///
    /// let scalar = ScalarAsVec(42);
    /// assert_eq!(Some(42), FunVecD1::at(&scalar, 7));
    /// assert_eq!(Some(42), FunVecD1::at(&scalar, 12));
    ///
    /// let empty_vec: EmptyVec<i32> = EmptyVec::new();
    /// assert_eq!(None, FunVecD1::at(&empty_vec, 7));
    /// assert_eq!(None, FunVecD1::at(&empty_vec, 12));
    /// ```
    fn at(&self, index: usize) -> Option<T>;

    /// Returns an iterator of elements of the vector for the given `indices`.
    ///
    /// `indices` can be any `Iterator` yielding `usize` indices.
    ///
    /// This allows to iterate over all funvec implementations in a unified way. Thanks to monomorphization, this abstraction does not have a performance penalty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_funvec::*;
    ///
    /// fn sum_values<V: FunVecD1<i32>, I: Iterator<Item = usize>>(vec: &V, indices: I) -> i32 {
    ///     vec.iter_over(indices).flatten().sum()
    /// }
    ///
    /// let stdvec = vec![10, 11, 12, 13];
    /// assert_eq!(23, sum_values(&stdvec, 1..3));
    ///
    /// let map = std::collections::HashMap::from_iter([(1, 10), (8, 20), (12, 200)].into_iter());
    /// assert_eq!(20, sum_values(&map, (1..10).filter(|x| x % 2 == 0)));
    ///
    /// let (s, t) = (0, 42);
    /// let closure = orx_closure::Capture((s, t))
    ///     .fun(|(s, t), i: usize| if i == *s { Some(10) } else if i == *t { Some(-1) } else { None });
    /// assert_eq!(9, sum_values(&closure, [0, 21, 42].iter().copied()));
    ///
    /// let scalar = ScalarAsVec(21);
    /// assert_eq!(42, sum_values(&scalar, 1..3));
    ///
    /// let empty_vec: EmptyVec<i32> = EmptyVec::new();
    /// assert_eq!(0, sum_values(&empty_vec, 1..3));
    /// ```
    fn iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = usize> + 'a;
}

/// Trait to provide abstraction over 1-dimensional vectors allowing reference access using indices.
///
/// Such an abstraction is particularly important in performance-critical algorithms both requiring flexibility through abstraction
/// over inputs and performance through monomorphization.
///
/// # Example
///
/// ```rust
/// use orx_funvec::*;
///
/// fn moving_average<V: FunVecRefD1<i32>>(observations: &V, period: usize) -> Option<i32> {
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
pub trait FunVecRefD1<T: ?Sized> {
    type Iter<'a, I>: Iterator<Item = Option<&'a T>> + 'a
    where
        T: 'a,
        Self: 'a,
        I: Iterator<Item = usize> + 'a;

    /// Returns a reference to the element at the given `index` or `None` if the position is empty.
    ///
    /// This allows to access elements of all funvec implementations in a unified way. Thanks to monomorphization, this abstraction does not have a performance penalty.
    ///
    /// Note that funvec's are different than, generalization of, traditional since the elements are not necessarily contagious:
    /// * there might exist values for indices 0 and 1;
    /// * positions 2 and 3 might be empty returning `None`;
    /// * and there might again be a value at position 4.
    ///
    /// Therefore, `ref_at` always returns an optional.
    ///
    /// # Example
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
    /// fn average_student_age<S: FunVecRefD1<Student>>(students: &S, ids: &HashSet<usize>) -> u32 {
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
    fn ref_at(&self, index: usize) -> Option<&T>;

    /// Returns an iterator yielding references to elements of the vector for the given `indices`.
    ///
    /// `indices` can be any `Iterator` yielding `usize` indices.
    ///
    /// This allows to iterate over all funvec implementations in a unified way. Thanks to monomorphization, this abstraction does not have a performance penalty.
    ///
    /// # Example
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
    /// fn average_student_age<S: FunVecRefD1<Student>>(students: &S, ids: &HashSet<usize>) -> u32 {
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
    fn ref_iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = usize> + 'a;
}
