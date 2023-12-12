/// Nothing but a wrapped value, a tuple struct, that allows to represent its internal scalar value as
/// an infinite-length vector having the same value at all positions.
///
/// # Examples
///
/// Say, for instance, we have a function requiring a vector, `FunVec<1, i32>`.
///
/// ```rust
/// use orx_funvec::*;
/// fn third_element<V: FunVec<1, i32>>(vec: &V) -> Option<i32> {
///     vec.at(2)
/// }
/// ```
///
/// We might often call this function with a `Vec<i32>`.
///
/// ```rust
/// # use orx_funvec::*;
/// # fn third_element<V: FunVec<1, i32>>(vec: &V) -> Option<i32> {
/// #     vec.at(2)
/// # }
/// let numbers = vec![1, 2, 3, 4, 5, 6];
/// assert_eq!(Some(3), third_element(&numbers));
/// ```
///
/// There might however be special cases where our input vector is all 42s.
/// Probably not 42s, but all zeros/ones vectors/matrices are often very useful.
/// Following would work:
///
/// ```rust
/// # use orx_funvec::*;
/// # fn third_element<V: FunVec<1, i32>>(vec: &V) -> Option<i32> {
/// #    vec.at(2)
/// # }
/// let numbers = vec![42, 42, 42, 42, 42];
/// assert_eq!(Some(42), third_element(&numbers));
/// ```
///
/// However, this would not be the best way to achieve this:
///
/// * we allocate a vector just to return 42, and
/// * we make method calls just to read 42 and lose compiler optimization potential.
///
/// We can instead use `ScalarAsVec` wrapper which implements `FunVec<1, _>`:
///
/// ```rust
/// # use orx_funvec::*;
/// # fn third_element<V: FunVec<1, i32>>(vec: &V) -> Option<i32> {
/// #    vec.at(2)
/// # }
/// let numbers = ScalarAsVec(42);
/// assert_eq!(Some(42), third_element(&numbers));
/// ```
///
/// Actually, `ScalarAsVec` implements `FunVec` for all dimensions:
///
/// ```rust
/// # use orx_funvec::*;
///
/// let numbers = ScalarAsVec(42);
///
/// assert_eq!(Some(42), numbers.at(3));
/// assert_eq!(Some(42), numbers.at([7, 2]));
/// assert_eq!(Some(42), numbers.at([14, 1, 0]));
/// assert_eq!(Some(42), numbers.at((4, 1, 3, 6))); // array or tuple indices can be used interchangeably
/// ```
pub struct ScalarAsVec<T>(pub T);
