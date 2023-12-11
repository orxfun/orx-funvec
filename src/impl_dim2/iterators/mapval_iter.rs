use crate::{funvec_d2::Ind, scalar_asvec::Scalar, FunVecD1};
use orx_closure::{Closure, ClosureOneOf2, ClosureOneOf3, ClosureOneOf4};
use std::{
    collections::{BTreeMap, HashMap},
    marker::PhantomData,
};

pub trait MapValD2<T: Clone + Copy> {
    fn get_val_by_key(&self, indices: Ind) -> Option<T>;
}

#[derive(derive_new::new)]
pub struct MapValD2Iter<'a, T, I, M>
where
    I: Iterator<Item = Ind> + 'a,
    M: MapValD2<T>,
    T: Clone + Copy,
{
    value: &'a M,
    indices_iter: I,
    ph: PhantomData<&'a T>,
}
impl<'a, T, I, M> Iterator for MapValD2Iter<'a, T, I, M>
where
    I: Iterator<Item = Ind> + 'a,
    M: MapValD2<T>,
    T: Clone + Copy,
{
    type Item = Option<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.indices_iter
            .next()
            .map(|indices| self.value.get_val_by_key(indices))
    }
}

// impl map-val
impl<T: Clone + Copy> MapValD2<T> for Scalar<T> {
    #[inline(always)]
    fn get_val_by_key(&self, _: Ind) -> Option<T> {
        self.0
    }
}
impl<T: Clone + Copy, V1: FunVecD1<T>> MapValD2<T> for Vec<V1> {
    #[inline(always)]
    fn get_val_by_key(&self, indices: Ind) -> Option<T> {
        self.get(indices.0).and_then(|x| x.val_at(indices.1))
    }
}
impl<const N: usize, T: Clone + Copy, V1: FunVecD1<T>> MapValD2<T> for [V1; N] {
    #[inline(always)]
    fn get_val_by_key(&self, indices: Ind) -> Option<T> {
        self.get(indices.0).and_then(|x| x.val_at(indices.1))
    }
}

impl<T: Clone + Copy, V1: FunVecD1<T>> MapValD2<T> for HashMap<usize, V1> {
    #[inline(always)]
    fn get_val_by_key(&self, indices: Ind) -> Option<T> {
        self.get(&indices.0).and_then(|x| x.val_at(indices.1))
    }
}
impl<T: Clone + Copy, V1: FunVecD1<T>> MapValD2<T> for BTreeMap<usize, V1> {
    #[inline(always)]
    fn get_val_by_key(&self, indices: Ind) -> Option<T> {
        self.get(&indices.0).and_then(|x| x.val_at(indices.1))
    }
}
#[cfg(any(feature = "impl_all", feature = "impl_indexmap"))]
impl<T: Clone + Copy, V1: FunVecD1<T>> MapValD2<T> for indexmap::IndexMap<usize, V1> {
    #[inline(always)]
    fn get_val_by_key(&self, indices: Ind) -> Option<T> {
        self.get(&indices.0).and_then(|x| x.val_at(indices.1))
    }
}

// non-recursive
impl<C1, T: Clone + Copy> MapValD2<T> for Closure<C1, Ind, Option<T>> {
    #[inline(always)]
    fn get_val_by_key(&self, indices: Ind) -> Option<T> {
        self.call(indices)
    }
}
impl<C1, C2, T: Clone + Copy> MapValD2<T> for ClosureOneOf2<C1, C2, Ind, Option<T>> {
    #[inline(always)]
    fn get_val_by_key(&self, indices: Ind) -> Option<T> {
        self.call(indices)
    }
}
impl<C1, C2, C3, T: Clone + Copy> MapValD2<T> for ClosureOneOf3<C1, C2, C3, Ind, Option<T>> {
    #[inline(always)]
    fn get_val_by_key(&self, indices: Ind) -> Option<T> {
        self.call(indices)
    }
}
impl<C1, C2, C3, C4, T: Clone + Copy> MapValD2<T>
    for ClosureOneOf4<C1, C2, C3, C4, Ind, Option<T>>
{
    #[inline(always)]
    fn get_val_by_key(&self, indices: Ind) -> Option<T> {
        self.call(indices)
    }
}

// non-recursive - only val
impl<T: Clone + Copy> MapValD2<T> for Box<dyn Fn(Ind) -> Option<T>> {
    #[inline(always)]
    fn get_val_by_key(&self, indices: Ind) -> Option<T> {
        (self)(indices)
    }
}

// ndarray
#[cfg(any(feature = "impl_all", feature = "impl_ndarray"))]
impl<T: Clone + Copy> MapValD2<T> for ndarray::Array2<T> {
    #[inline(always)]
    fn get_val_by_key(&self, key: Ind) -> Option<T> {
        self.get([key.0, key.1]).copied()
    }
}
