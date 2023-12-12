use std::marker::PhantomData;

/// A zero-sized empty vector which returns None for all indices.
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
/// There might however be special cases where our input vector is empty.
/// Following could still work:
///
/// ```rust
/// # use orx_funvec::*;
/// # fn third_element<V: FunVec<1, i32>>(vec: &V) -> Option<i32> {
/// #    vec.at(2)
/// # }
/// let numbers = vec![];
/// assert_eq!(None, third_element(&numbers));
/// ```
///
/// However, this would not be the best way to achieve this:
/// * since the `third_element` is already generic, we can take complete benefit of monomorphization and inlining by using a specific type that does nothing but returns `None` for all indices.
///
/// We can instead use `EmptyVec` for this purpose which implements `FunVec<1, _>`.
///
/// ```rust
/// # use orx_funvec::*;
/// # fn third_element<V: FunVec<1, i32>>(vec: &V) -> Option<i32> {
/// #    vec.at(2)
/// # }
/// let numbers = EmptyVec::default();
/// assert_eq!(None, third_element(&numbers));
/// ```
///
/// Actually, `EmptyVec` implements `FunVec` for all dimensions::
///
/// ```rust
/// # use orx_funvec::*;
///
/// let numbers: EmptyVec<&str> = EmptyVec::default();
///
/// assert_eq!(None, numbers.at(3));
/// assert_eq!(None, numbers.at([7, 2]));
/// assert_eq!(None, numbers.at([14, 1, 0]));
/// assert_eq!(None, numbers.at((4, 1, 3, 6))); // array or tuple indices can be used interchangeably
/// ```
#[derive(derive_new::new, Default)]
pub struct EmptyVec<T: ?Sized>(PhantomData<T>);
