use crate::{
    funvec_d3::FunVecRefD3,
    funvec_d4::Ind,
    vectors_by_scalars::{EmptyVec, ScalarAsVec},
};
use orx_closure::{ClosureOptRef, ClosureOptRefOneOf2, ClosureOptRefOneOf3, ClosureOptRefOneOf4};
use std::{
    collections::{BTreeMap, HashMap},
    marker::PhantomData,
};

pub trait MapRefD4<T: ?Sized> {
    fn get_ref_at(&self, key: Ind) -> Option<&T>;
}

#[derive(derive_new::new)]
pub struct MapRefD4Iter<'a, T, I, M>
where
    I: Iterator<Item = Ind> + 'a,
    M: MapRefD4<T>,
    T: ?Sized,
{
    value: &'a M,
    indices_iter: I,
    ph: PhantomData<&'a T>,
}
impl<'a, T, I, M> Iterator for MapRefD4Iter<'a, T, I, M>
where
    I: Iterator<Item = Ind> + 'a,
    M: MapRefD4<T>,
    T: ?Sized,
{
    type Item = Option<&'a T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.indices_iter.next().map(|i| self.value.get_ref_at(i))
    }
}

// impl map-ref
impl<T> MapRefD4<T> for ScalarAsVec<T> {
    fn get_ref_at(&self, _: Ind) -> Option<&T> {
        Some(&self.0)
    }
}
impl<T> MapRefD4<T> for EmptyVec<T> {
    fn get_ref_at(&self, _: Ind) -> Option<&T> {
        None
    }
}
impl<T, V3: FunVecRefD3<T>> MapRefD4<T> for Vec<V3> {
    #[inline(always)]
    fn get_ref_at(&self, indices: Ind) -> Option<&T> {
        self.get(indices.0)
            .and_then(|x| x.ref_at(indices.1, indices.2, indices.3))
    }
}
impl<const N: usize, T, V3: FunVecRefD3<T>> MapRefD4<T> for [V3; N] {
    #[inline(always)]
    fn get_ref_at(&self, indices: Ind) -> Option<&T> {
        self.get(indices.0)
            .and_then(|x| x.ref_at(indices.1, indices.2, indices.3))
    }
}

impl<T, V3: FunVecRefD3<T>> MapRefD4<T> for HashMap<usize, V3> {
    #[inline(always)]
    fn get_ref_at(&self, indices: Ind) -> Option<&T> {
        self.get(&indices.0)
            .and_then(|x| x.ref_at(indices.1, indices.2, indices.3))
    }
}
impl<T, V3: FunVecRefD3<T>> MapRefD4<T> for BTreeMap<usize, V3> {
    #[inline(always)]
    fn get_ref_at(&self, indices: Ind) -> Option<&T> {
        self.get(&indices.0)
            .and_then(|x| x.ref_at(indices.1, indices.2, indices.3))
    }
}
#[cfg(any(feature = "impl_all", feature = "impl_indexmap"))]
impl<T, V3: FunVecRefD3<T>> MapRefD4<T> for indexmap::IndexMap<usize, V3> {
    #[inline(always)]
    fn get_ref_at(&self, indices: Ind) -> Option<&T> {
        self.get(&indices.0)
            .and_then(|x| x.ref_at(indices.1, indices.2, indices.3))
    }
}
impl<T> MapRefD4<T> for HashMap<Ind, T> {
    #[inline(always)]
    fn get_ref_at(&self, indices: Ind) -> Option<&T> {
        self.get(&indices)
    }
}
impl<T> MapRefD4<T> for BTreeMap<Ind, T> {
    #[inline(always)]
    fn get_ref_at(&self, indices: Ind) -> Option<&T> {
        self.get(&indices)
    }
}
#[cfg(any(feature = "impl_all", feature = "impl_indexmap"))]
impl<T> MapRefD4<T> for indexmap::IndexMap<Ind, T> {
    #[inline(always)]
    fn get_ref_at(&self, indices: Ind) -> Option<&T> {
        self.get(&indices)
    }
}

// non-recursive
impl<C1, T: ?Sized> MapRefD4<T> for ClosureOptRef<C1, Ind, T> {
    #[inline(always)]
    fn get_ref_at(&self, indices: Ind) -> Option<&T> {
        self.call(indices)
    }
}
impl<C1, C2, T: ?Sized> MapRefD4<T> for ClosureOptRefOneOf2<C1, C2, Ind, T> {
    #[inline(always)]
    fn get_ref_at(&self, indices: Ind) -> Option<&T> {
        self.call(indices)
    }
}
impl<C1, C2, C3, T: ?Sized> MapRefD4<T> for ClosureOptRefOneOf3<C1, C2, C3, Ind, T> {
    #[inline(always)]
    fn get_ref_at(&self, indices: Ind) -> Option<&T> {
        self.call(indices)
    }
}
impl<C1, C2, C3, C4, T: ?Sized> MapRefD4<T> for ClosureOptRefOneOf4<C1, C2, C3, C4, Ind, T> {
    #[inline(always)]
    fn get_ref_at(&self, indices: Ind) -> Option<&T> {
        self.call(indices)
    }
}

// ndarray
#[cfg(any(feature = "impl_all", feature = "impl_ndarray"))]
impl<T> MapRefD4<T> for ndarray::Array4<T> {
    #[inline(always)]
    fn get_ref_at(&self, key: Ind) -> Option<&T> {
        self.get([key.0, key.1, key.2, key.3])
    }
}
