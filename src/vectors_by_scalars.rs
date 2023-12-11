use std::marker::PhantomData;

/// Nothing but a wrapped value, a tuple struct, representing its internal value as a scalar that can be treated as a vector.
///
/// # Examples
///
/// Say, for instance, we have a function requiring a vector, `FunVecD1<i32>`.
///
/// ```rust
/// use orx_funvec::*;
/// fn third_element<V: FunVecD1<i32>>(vec: &V) -> Option<i32> {
///     vec.at(2)
/// }
/// ```
///
/// We might often call this function with a `Vec<i32>`.
///
/// ```rust
/// # use orx_funvec::*;
/// # fn third_element<V: FunVecD1<i32>>(vec: &V) -> Option<i32> {
/// #     vec.at(2)
/// # }
/// let numbers = vec![1, 2, 3, 4, 5, 6];
/// assert_eq!(Some(3), third_element(&numbers));
/// ```
///
/// There might however be special cases where our input vector now is all 42s.
/// Note that it is common to have all zeros or ones vectors/matrices.
/// Following could still work:
///
/// ```rust
/// # use orx_funvec::*;
/// # fn third_element<V: FunVecD1<i32>>(vec: &V) -> Option<i32> {
/// #    vec.at(2)
/// # }
/// let numbers = vec![42, 42, 42, 42, 42];
/// assert_eq!(Some(42), third_element(&numbers));
/// ```
///
/// However, this would not be the best way to achieve this:
/// * we allocate a vector just to return 42, and
/// * we make a method calls just to read 42 and lose compiler optimization potential.
///
/// We can instead use `ScalarAsVec` wrapper which implements `FunVecD1`:
/// ```rust
/// # use orx_funvec::*;
/// # fn third_element<V: FunVecD1<i32>>(vec: &V) -> Option<i32> {
/// #    vec.at(2)
/// # }
/// let numbers = ScalarAsVec(42);
/// assert_eq!(Some(42), third_element(&numbers));
/// ```
///
/// `ScalarAsVec` actually implements funvec's for all dimensions:
///
/// ```rust
/// # use orx_funvec::*;
///
/// let numbers = ScalarAsVec(42);
///
/// assert_eq!(Some(42), FunVecD1::at(&numbers, 3));
/// assert_eq!(Some(42), FunVecD2::at(&numbers, 7, 2));
/// assert_eq!(Some(42), FunVecD3::at(&numbers, 14, 1, 0));
/// assert_eq!(Some(42), FunVecD4::at(&numbers, 4, 1, 3, 6));
/// ```
pub struct ScalarAsVec<T>(pub T);

/// A zero-sized empty vector which returns None for all indices.
///
/// # Examples
///
/// Say, for instance, we have a function requiring a vector, `FunVecD1<i32>`.
///
/// ```rust
/// use orx_funvec::*;
/// fn third_element<V: FunVecD1<i32>>(vec: &V) -> Option<i32> {
///     vec.at(2)
/// }
/// ```
/// We might often call this function with a `Vec<i32>`.
///
/// ```rust
/// # use orx_funvec::*;
/// # fn third_element<V: FunVecD1<i32>>(vec: &V) -> Option<i32> {
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
/// # fn third_element<V: FunVecD1<i32>>(vec: &V) -> Option<i32> {
/// #    vec.at(2)
/// # }
/// let numbers = vec![];
/// assert_eq!(None, third_element(&numbers));
/// ```
///
/// However, this would not be the best way to achieve this:
/// * since the `third_element` is already generic, we can take complete benefit of monomorphization and inlining by using a specific type that does nothing but returns `None` for all indices.
///
/// We can instead use `EmptyVec` for this purpose which implements `FunVecD1`:
/// ```rust
/// # use orx_funvec::*;
/// # fn third_element<V: FunVecD1<i32>>(vec: &V) -> Option<i32> {
/// #    vec.at(2)
/// # }
/// let numbers = EmptyVec::default();
/// assert_eq!(None, third_element(&numbers));
/// ```
///
/// `EmptyVec` actually implements funvec's for all dimensions:
///
/// ```rust
/// # use orx_funvec::*;
///
/// let numbers: EmptyVec<&str> = EmptyVec::default();
///
/// assert_eq!(None, FunVecD1::at(&numbers, 3));
/// assert_eq!(None, FunVecD2::at(&numbers, 7, 2));
/// assert_eq!(None, FunVecD3::at(&numbers, 14, 1, 0));
/// assert_eq!(None, FunVecD4::at(&numbers, 4, 1, 3, 6));
/// ```
#[derive(derive_new::new, Default)]
pub struct EmptyVec<T>(PhantomData<T>);
