pub(crate) type Ind = (usize, usize, usize, usize);

/// Trait to provide abstraction over 4-dimensional vectors allowing access using indices.
///
/// Such an abstraction is particularly important in performance-critical algorithms both requiring flexibility through abstraction
/// over inputs and performance through monomorphization.
///
/// # Example
///
/// See 2-dimensional examples here [crate::FunVecD2].
pub trait FunVecD4<T> {
    type Iter<'a, I>: Iterator<Item = Option<T>> + 'a
    where
        T: 'a,
        Self: 'a,
        I: Iterator<Item = Ind> + 'a;

    /// Returns the value at the given  given indices (`i`, `j`, `k`, `l`) or `None` if the position is empty.
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
    /// See 2-dimensional examples here [crate::FunVecD2].
    fn at(&self, i: usize, j: usize, k: usize, l: usize) -> Option<T>;

    /// Returns an iterator of elements of the vector for the given `indices`.
    ///
    /// `indices` can be any `Iterator` yielding `(usize, usize, usize, usize)` indices.
    ///
    /// This allows to iterate over all funvec implementations in a unified way. Thanks to monomorphization, this abstraction does not have a performance penalty.
    ///
    /// # Example
    ///
    /// See 2-dimensional examples here [crate::FunVecD2].
    fn iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = Ind> + 'a;
}

/// Trait to provide abstraction over 4-dimensional vectors allowing reference access using indices.
///
/// Such an abstraction is particularly important in performance-critical algorithms both requiring flexibility through abstraction
/// over inputs and performance through monomorphization.
///
/// # Example
///
/// See 2-dimensional examples here [crate::FunVecRefD2].
pub trait FunVecRefD4<T: ?Sized> {
    type Iter<'a, I>: Iterator<Item = Option<&'a T>> + 'a
    where
        T: 'a,
        Self: 'a,
        I: Iterator<Item = Ind> + 'a;

    /// Returns a reference to the element at the given indices (`i`, `j`, `k`, `l`) or `None` if the position is empty.
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
    /// See 2-dimensional examples here [crate::FunVecRefD2].
    fn ref_at(&self, i: usize, j: usize, k: usize, l: usize) -> Option<&T>;

    /// Returns an iterator yielding references to elements of the vector for the given `indices`.
    ///
    /// `indices` can be any `Iterator` yielding `(usize, usize, usize, usize)` indices.
    ///
    /// This allows to iterate over all funvec implementations in a unified way. Thanks to monomorphization, this abstraction does not have a performance penalty.
    ///
    /// # Example
    ///
    /// See 2-dimensional examples here [crate::FunVecRefD2].
    fn ref_iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = Ind> + 'a;
}
