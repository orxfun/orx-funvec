/// A convenience trait to allow accepting and interchangeably using indices as arrays or tuples.
///
/// A struct implementing `IntoIndex<DIM>` can be converted into and index represented as `[usize; DIM]`, such as:
///
/// * `usize`, `(usize)`, `[usize; 1]` implement `IntoIndex<1>`, and hence, can be converted into `[usize; 1]`;
/// * `(usize, usize)`, `[usize; 2]` implement `IntoIndex<2>`, and hence, can be converted into `[usize; 2]`;
/// * ...
pub trait IntoIndex<const DIM: usize> {
    /// Converts the value into an index represented as `[usize; DIM]`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_funvec::*;
    ///
    /// let i = 2;
    /// let index = i.into_index();
    /// assert_eq!(index, [2]);
    ///
    /// let ij = (3, 4);
    /// let index = ij.into_index();
    /// assert_eq!(index, [3, 4]);
    /// ```
    fn into_index(self) -> [usize; DIM];
}
impl<const DIM: usize> IntoIndex<DIM> for [usize; DIM] {
    fn into_index(self) -> [usize; DIM] {
        self
    }
}

/// A convenience trait to allow extending `FunVec` implementations.
///
/// A struct implementing `FromIndex<DIM>` can be created from `[usize; DIM]`, such as:
///
/// * `usize`, `(usize)`, `[usize; 1]` implement `FromIndex<1>`, and hence, can be created from `[usize; 1]`;
/// * `(usize, usize)`, `[usize; 2]` implement `FromIndex<2>`, and hence, can be created from `[usize; 2]`;
/// * ...
pub trait FromIndex<const DIM: usize> {
    /// Converts the index `[usize; DIM]` into value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_funvec::*;
    ///
    /// let index = [2];
    /// let i = usize::from_index(index);
    /// assert_eq!(i, 2);
    ///
    /// let index = [3, 4];
    /// let (i, j) = <(usize, usize)>::from_index(index);
    /// assert_eq!(i, 3);
    /// assert_eq!(j, 4);
    /// ```
    fn from_index(index: [usize; DIM]) -> Self;
}
impl<const DIM: usize> FromIndex<DIM> for [usize; DIM] {
    fn from_index(index: [usize; DIM]) -> Self {
        index
    }
}
