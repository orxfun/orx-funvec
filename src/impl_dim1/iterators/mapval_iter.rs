use crate::vectors_by_scalars::{EmptyVec, ScalarAsVec};
use orx_closure::{Closure, ClosureOneOf2, ClosureOneOf3, ClosureOneOf4};
use std::collections::{BTreeMap, HashMap};
use std::marker::PhantomData;

/// Trait to unify access by index.
///
/// Any type implementing `MapVal` auto-implements `FunVecD1`.
///
/// # Example
///
/// ```rust
/// use orx_funvec::*;
///
/// struct DivisibleBy {
///     divider: usize,
/// }
/// impl MapVal<usize> for DivisibleBy {
///     fn get_value_at(&self, index: usize) -> Option<usize> {
///         if index % self.divider == 0 {
///             Some(index)
///         } else {
///             None
///         }
///     }
/// }
///
/// let evens = DivisibleBy { divider: 2 };
///
/// // as MapVal
/// assert_eq!(Some(4), evens.get_value_at(4));
/// assert_eq!(None, evens.get_value_at(1));
///
/// // as FunVec
/// assert_eq!(Some(4), evens.at(4));
/// assert_eq!(None, evens.at(1));
///
/// let sum_of_evens_up_to_ten: usize = evens.iter_over(0..10).flatten().sum();
/// assert_eq!(20, sum_of_evens_up_to_ten);
/// ```
pub trait MapVal<T: Clone + Copy> {
    fn get_value_at(&self, index: usize) -> Option<T>;
}

#[derive(derive_new::new)]
pub struct MapValIter<'a, T, I, M>
where
    I: Iterator<Item = usize> + 'a,
    M: MapVal<T>,
    T: Clone + Copy,
{
    value: &'a M,
    indices_iter: I,
    ph: PhantomData<&'a T>,
}
impl<'a, T, I, M> Iterator for MapValIter<'a, T, I, M>
where
    I: Iterator<Item = usize> + 'a,
    M: MapVal<T>,
    T: Clone + Copy,
{
    type Item = Option<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.indices_iter.next().map(|i| self.value.get_value_at(i))
    }
}

// impl map-val
impl<T: Clone + Copy> MapVal<T> for ScalarAsVec<T> {
    #[inline(always)]
    fn get_value_at(&self, _: usize) -> Option<T> {
        Some(self.0)
    }
}
impl<T: Clone + Copy> MapVal<T> for EmptyVec<T> {
    fn get_value_at(&self, _: usize) -> Option<T> {
        None
    }
}
impl<T: Clone + Copy> MapVal<T> for Vec<T> {
    #[inline(always)]
    fn get_value_at(&self, i: usize) -> Option<T> {
        self.get(i).copied()
    }
}
impl<const N: usize, T: Clone + Copy> MapVal<T> for [T; N] {
    #[inline(always)]
    fn get_value_at(&self, i: usize) -> Option<T> {
        self.get(i).copied()
    }
}
impl<Capture, T: Clone + Copy> MapVal<T> for Closure<Capture, usize, Option<T>> {
    #[inline(always)]
    fn get_value_at(&self, key: usize) -> Option<T> {
        self.call(key)
    }
}
impl<C1, C2, T: Clone + Copy> MapVal<T> for ClosureOneOf2<C1, C2, usize, Option<T>> {
    #[inline(always)]
    fn get_value_at(&self, key: usize) -> Option<T> {
        self.call(key)
    }
}
impl<C1, C2, C3, T: Clone + Copy> MapVal<T> for ClosureOneOf3<C1, C2, C3, usize, Option<T>> {
    #[inline(always)]
    fn get_value_at(&self, key: usize) -> Option<T> {
        self.call(key)
    }
}
impl<C1, C2, C3, C4, T: Clone + Copy> MapVal<T>
    for ClosureOneOf4<C1, C2, C3, C4, usize, Option<T>>
{
    #[inline(always)]
    fn get_value_at(&self, key: usize) -> Option<T> {
        self.call(key)
    }
}

impl<T: Clone + Copy> MapVal<T> for HashMap<usize, T> {
    #[inline(always)]
    fn get_value_at(&self, key: usize) -> Option<T> {
        self.get(&key).copied()
    }
}
impl<T: Clone + Copy> MapVal<T> for BTreeMap<usize, T> {
    #[inline(always)]
    fn get_value_at(&self, key: usize) -> Option<T> {
        self.get(&key).copied()
    }
}
#[cfg(any(feature = "impl_all", feature = "impl_indexmap"))]
impl<T: Clone + Copy> MapVal<T> for indexmap::IndexMap<usize, T> {
    #[inline(always)]
    fn get_value_at(&self, key: usize) -> Option<T> {
        self.get(&key).copied()
    }
}

// only val
impl<T: Clone + Copy> MapVal<T> for Box<dyn Fn(usize) -> Option<T>> {
    #[inline(always)]
    fn get_value_at(&self, key: usize) -> Option<T> {
        (self)(key)
    }
}

// ndarray
#[cfg(any(feature = "impl_all", feature = "impl_ndarray"))]
impl<T: Clone + Copy> MapVal<T> for ndarray::Array1<T> {
    #[inline(always)]
    fn get_value_at(&self, key: usize) -> Option<T> {
        self.get([key]).copied()
    }
}
