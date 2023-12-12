use crate::vectors_by_scalars::{EmptyVec, ScalarAsVec};
use orx_closure::{ClosureOptRef, ClosureOptRefOneOf2, ClosureOptRefOneOf3, ClosureOptRefOneOf4};
use std::collections::{BTreeMap, HashMap};
use std::marker::PhantomData;

pub trait MapRef<T: ?Sized> {
    fn get_ref_at(&self, key: usize) -> Option<&T>;
}

#[derive(derive_new::new)]
pub struct MapRefIter<'a, T, I, M>
where
    I: Iterator<Item = usize> + 'a,
    M: MapRef<T>,
    T: ?Sized,
{
    value: &'a M,
    indices_iter: I,
    ph: PhantomData<&'a T>,
}
impl<'a, T, I, M> Iterator for MapRefIter<'a, T, I, M>
where
    I: Iterator<Item = usize> + 'a,
    M: MapRef<T>,
    T: ?Sized,
{
    type Item = Option<&'a T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.indices_iter.next().map(|i| self.value.get_ref_at(i))
    }
}

// impl map-ref
impl<T> MapRef<T> for ScalarAsVec<T> {
    fn get_ref_at(&self, _: usize) -> Option<&T> {
        Some(&self.0)
    }
}
impl<T> MapRef<T> for EmptyVec<T> {
    fn get_ref_at(&self, _: usize) -> Option<&T> {
        None
    }
}
impl<T> MapRef<T> for Vec<T> {
    #[inline(always)]
    fn get_ref_at(&self, i: usize) -> Option<&T> {
        self.get(i)
    }
}
impl<const N: usize, T> MapRef<T> for [T; N] {
    #[inline(always)]
    fn get_ref_at(&self, i: usize) -> Option<&T> {
        self.get(i)
    }
}

impl<C1, T: ?Sized> MapRef<T> for ClosureOptRef<C1, usize, T> {
    #[inline(always)]
    fn get_ref_at(&self, key: usize) -> Option<&T> {
        self.call(key)
    }
}
impl<C1, C2, T: ?Sized> MapRef<T> for ClosureOptRefOneOf2<C1, C2, usize, T> {
    #[inline(always)]
    fn get_ref_at(&self, key: usize) -> Option<&T> {
        self.call(key)
    }
}
impl<C1, C2, C3, T: ?Sized> MapRef<T> for ClosureOptRefOneOf3<C1, C2, C3, usize, T> {
    #[inline(always)]
    fn get_ref_at(&self, key: usize) -> Option<&T> {
        self.call(key)
    }
}
impl<C1, C2, C3, C4, T: ?Sized> MapRef<T> for ClosureOptRefOneOf4<C1, C2, C3, C4, usize, T> {
    #[inline(always)]
    fn get_ref_at(&self, key: usize) -> Option<&T> {
        self.call(key)
    }
}

impl<T> MapRef<T> for HashMap<usize, T> {
    #[inline(always)]
    fn get_ref_at(&self, key: usize) -> Option<&T> {
        self.get(&key)
    }
}
impl<T> MapRef<T> for BTreeMap<usize, T> {
    #[inline(always)]
    fn get_ref_at(&self, key: usize) -> Option<&T> {
        self.get(&key)
    }
}
#[cfg(any(feature = "impl_all", feature = "impl_indexmap"))]
impl<T> MapRef<T> for indexmap::IndexMap<usize, T> {
    #[inline(always)]
    fn get_ref_at(&self, key: usize) -> Option<&T> {
        self.get(&key)
    }
}

// ndarray
#[cfg(any(feature = "impl_all", feature = "impl_ndarray"))]
impl<T> MapRef<T> for ndarray::Array1<T> {
    #[inline(always)]
    fn get_ref_at(&self, key: usize) -> Option<&T> {
        self.get([key])
    }
}
