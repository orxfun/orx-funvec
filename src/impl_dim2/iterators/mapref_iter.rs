use crate::{funvec_d2::Ind, scalar_asvec::Scalar, FunVecD1Ref};
use indexmap::IndexMap;
use orx_closure::{ClosureOptRef, ClosureOptRefOneOf2, ClosureOptRefOneOf3, ClosureOptRefOneOf4};
use std::{
    collections::{BTreeMap, HashMap},
    marker::PhantomData,
};

pub trait MapRefD2<T: ?Sized> {
    fn get_ref_by_key(&self, key: Ind) -> Option<&T>;
}

#[derive(derive_new::new)]
pub struct MapRefD2Iter<'a, T, I, M>
where
    I: Iterator<Item = Ind> + 'a,
    M: MapRefD2<T>,
    T: ?Sized,
{
    value: &'a M,
    indices_iter: I,
    ph: PhantomData<&'a T>,
}
impl<'a, T, I, M> Iterator for MapRefD2Iter<'a, T, I, M>
where
    I: Iterator<Item = Ind> + 'a,
    M: MapRefD2<T>,
    T: ?Sized,
{
    type Item = Option<&'a T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.indices_iter
            .next()
            .map(|i| self.value.get_ref_by_key(i))
    }
}

// impl map-ref
impl<T> MapRefD2<T> for Scalar<T> {
    fn get_ref_by_key(&self, _: Ind) -> Option<&T> {
        self.0.as_ref()
    }
}
impl<T, V1: FunVecD1Ref<T>> MapRefD2<T> for Vec<V1> {
    #[inline(always)]
    fn get_ref_by_key(&self, indices: Ind) -> Option<&T> {
        self.get(indices.0).and_then(|x| x.ref_at(indices.1))
    }
}
impl<const N: usize, T, V1: FunVecD1Ref<T>> MapRefD2<T> for [V1; N] {
    #[inline(always)]
    fn get_ref_by_key(&self, indices: Ind) -> Option<&T> {
        self.get(indices.0).and_then(|x| x.ref_at(indices.1))
    }
}

impl<T, V1: FunVecD1Ref<T>> MapRefD2<T> for HashMap<usize, V1> {
    #[inline(always)]
    fn get_ref_by_key(&self, indices: Ind) -> Option<&T> {
        self.get(&indices.0).and_then(|x| x.ref_at(indices.1))
    }
}
impl<T, V1: FunVecD1Ref<T>> MapRefD2<T> for BTreeMap<usize, V1> {
    #[inline(always)]
    fn get_ref_by_key(&self, indices: Ind) -> Option<&T> {
        self.get(&indices.0).and_then(|x| x.ref_at(indices.1))
    }
}
impl<T, V1: FunVecD1Ref<T>> MapRefD2<T> for IndexMap<usize, V1> {
    #[inline(always)]
    fn get_ref_by_key(&self, indices: Ind) -> Option<&T> {
        self.get(&indices.0).and_then(|x| x.ref_at(indices.1))
    }
}

// non-recursive
impl<C1, T: ?Sized> MapRefD2<T> for ClosureOptRef<C1, Ind, T> {
    #[inline(always)]
    fn get_ref_by_key(&self, indices: Ind) -> Option<&T> {
        self.call(indices)
    }
}
impl<C1, C2, T: ?Sized> MapRefD2<T> for ClosureOptRefOneOf2<C1, C2, Ind, T> {
    #[inline(always)]
    fn get_ref_by_key(&self, indices: Ind) -> Option<&T> {
        self.call(indices)
    }
}
impl<C1, C2, C3, T: ?Sized> MapRefD2<T> for ClosureOptRefOneOf3<C1, C2, C3, Ind, T> {
    #[inline(always)]
    fn get_ref_by_key(&self, indices: Ind) -> Option<&T> {
        self.call(indices)
    }
}
impl<C1, C2, C3, C4, T: ?Sized> MapRefD2<T> for ClosureOptRefOneOf4<C1, C2, C3, C4, Ind, T> {
    #[inline(always)]
    fn get_ref_by_key(&self, indices: Ind) -> Option<&T> {
        self.call(indices)
    }
}

// ndarray
#[cfg(feature = "ndarray")]
impl<T> MapRefD2<T> for ndarray::Array2<T> {
    #[inline(always)]
    fn get_ref_by_key(&self, key: Ind) -> Option<&T> {
        self.get([key.0, key.1])
    }
}
