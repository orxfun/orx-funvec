use crate::scalar_asvec::Scalar;
use indexmap::IndexMap;
use orx_closure::{Closure, ClosureOneOf2, ClosureOneOf3, ClosureOneOf4};
use std::collections::{BTreeMap, HashMap};
use std::marker::PhantomData;

pub trait MapVal<T: Clone + Copy> {
    fn get_val_by_key(&self, key: usize) -> Option<T>;
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
        self.indices_iter
            .next()
            .map(|i| self.value.get_val_by_key(i))
    }
}

// impl map-val
impl<T: Clone + Copy> MapVal<T> for Scalar<T> {
    #[inline(always)]
    fn get_val_by_key(&self, _: usize) -> Option<T> {
        self.0
    }
}
impl<T: Clone + Copy> MapVal<T> for Vec<T> {
    #[inline(always)]
    fn get_val_by_key(&self, i: usize) -> Option<T> {
        self.get(i).copied()
    }
}
impl<const N: usize, T: Clone + Copy> MapVal<T> for [T; N] {
    #[inline(always)]
    fn get_val_by_key(&self, i: usize) -> Option<T> {
        self.get(i).copied()
    }
}
impl<Capture, T: Clone + Copy> MapVal<T> for Closure<Capture, usize, Option<T>> {
    #[inline(always)]
    fn get_val_by_key(&self, key: usize) -> Option<T> {
        self.call(key)
    }
}
impl<C1, C2, T: Clone + Copy> MapVal<T> for ClosureOneOf2<C1, C2, usize, Option<T>> {
    #[inline(always)]
    fn get_val_by_key(&self, key: usize) -> Option<T> {
        self.call(key)
    }
}
impl<C1, C2, C3, T: Clone + Copy> MapVal<T> for ClosureOneOf3<C1, C2, C3, usize, Option<T>> {
    #[inline(always)]
    fn get_val_by_key(&self, key: usize) -> Option<T> {
        self.call(key)
    }
}
impl<C1, C2, C3, C4, T: Clone + Copy> MapVal<T>
    for ClosureOneOf4<C1, C2, C3, C4, usize, Option<T>>
{
    #[inline(always)]
    fn get_val_by_key(&self, key: usize) -> Option<T> {
        self.call(key)
    }
}

impl<T: Clone + Copy> MapVal<T> for HashMap<usize, T> {
    #[inline(always)]
    fn get_val_by_key(&self, key: usize) -> Option<T> {
        self.get(&key).copied()
    }
}
impl<T: Clone + Copy> MapVal<T> for BTreeMap<usize, T> {
    #[inline(always)]
    fn get_val_by_key(&self, key: usize) -> Option<T> {
        self.get(&key).copied()
    }
}
impl<T: Clone + Copy> MapVal<T> for IndexMap<usize, T> {
    #[inline(always)]
    fn get_val_by_key(&self, key: usize) -> Option<T> {
        self.get(&key).copied()
    }
}

// only val
impl<T: Clone + Copy> MapVal<T> for Box<dyn Fn(usize) -> Option<T>> {
    #[inline(always)]
    fn get_val_by_key(&self, key: usize) -> Option<T> {
        (self)(key)
    }
}

// ndarray
#[cfg(feature = "impl_ndarray")]
impl<T: Clone + Copy> MapVal<T> for ndarray::Array1<T> {
    #[inline(always)]
    fn get_val_by_key(&self, key: usize) -> Option<T> {
        self.get([key]).copied()
    }
}
