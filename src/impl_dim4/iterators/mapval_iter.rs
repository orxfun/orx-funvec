use crate::{
    funvec_d3::FunVecD3,
    funvec_d4::Ind,
    vectors_by_scalars::{EmptyVec, ScalarAsVec},
};
use orx_closure::{Closure, ClosureOneOf2, ClosureOneOf3, ClosureOneOf4};
use std::{
    collections::{BTreeMap, HashMap},
    marker::PhantomData,
};

pub trait MapValD4<T: Clone + Copy> {
    fn get_value_at(&self, indices: Ind) -> Option<T>;
}

#[derive(derive_new::new)]
pub struct MapValD4Iter<'a, T, I, M>
where
    I: Iterator<Item = Ind> + 'a,
    M: MapValD4<T>,
    T: Clone + Copy,
{
    value: &'a M,
    indices_iter: I,
    ph: PhantomData<&'a T>,
}
impl<'a, T, I, M> Iterator for MapValD4Iter<'a, T, I, M>
where
    I: Iterator<Item = Ind> + 'a,
    M: MapValD4<T>,
    T: Clone + Copy,
{
    type Item = Option<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.indices_iter
            .next()
            .map(|indices| self.value.get_value_at(indices))
    }
}

// impl map-val
impl<T: Clone + Copy> MapValD4<T> for ScalarAsVec<T> {
    #[inline(always)]
    fn get_value_at(&self, _: Ind) -> Option<T> {
        Some(self.0)
    }
}
impl<T: Clone + Copy> MapValD4<T> for EmptyVec<T> {
    fn get_value_at(&self, _: Ind) -> Option<T> {
        None
    }
}
impl<T: Clone + Copy, V3: FunVecD3<T>> MapValD4<T> for Vec<V3> {
    #[inline(always)]
    fn get_value_at(&self, indices: Ind) -> Option<T> {
        self.get(indices.0)
            .and_then(|x| x.at(indices.1, indices.2, indices.3))
    }
}
impl<const N: usize, T: Clone + Copy, V3: FunVecD3<T>> MapValD4<T> for [V3; N] {
    #[inline(always)]
    fn get_value_at(&self, indices: Ind) -> Option<T> {
        self.get(indices.0)
            .and_then(|x| x.at(indices.1, indices.2, indices.3))
    }
}

impl<T: Clone + Copy, V3: FunVecD3<T>> MapValD4<T> for HashMap<usize, V3> {
    #[inline(always)]
    fn get_value_at(&self, indices: Ind) -> Option<T> {
        self.get(&indices.0)
            .and_then(|x| x.at(indices.1, indices.2, indices.3))
    }
}
impl<T: Clone + Copy, V3: FunVecD3<T>> MapValD4<T> for BTreeMap<usize, V3> {
    #[inline(always)]
    fn get_value_at(&self, indices: Ind) -> Option<T> {
        self.get(&indices.0)
            .and_then(|x| x.at(indices.1, indices.2, indices.3))
    }
}
#[cfg(any(feature = "impl_all", feature = "impl_indexmap"))]
impl<T: Clone + Copy, V3: FunVecD3<T>> MapValD4<T> for indexmap::IndexMap<usize, V3> {
    #[inline(always)]
    fn get_value_at(&self, indices: Ind) -> Option<T> {
        self.get(&indices.0)
            .and_then(|x| x.at(indices.1, indices.2, indices.3))
    }
}
impl<T: Clone + Copy> MapValD4<T> for HashMap<Ind, T> {
    #[inline(always)]
    fn get_value_at(&self, indices: Ind) -> Option<T> {
        self.get(&indices).copied()
    }
}
impl<T: Clone + Copy> MapValD4<T> for BTreeMap<Ind, T> {
    #[inline(always)]
    fn get_value_at(&self, indices: Ind) -> Option<T> {
        self.get(&indices).copied()
    }
}
#[cfg(any(feature = "impl_all", feature = "impl_indexmap"))]
impl<T: Clone + Copy> MapValD4<T> for indexmap::IndexMap<Ind, T> {
    #[inline(always)]
    fn get_value_at(&self, indices: Ind) -> Option<T> {
        self.get(&indices).copied()
    }
}

// non-recursive
impl<C1, T: Clone + Copy> MapValD4<T> for Closure<C1, Ind, Option<T>> {
    #[inline(always)]
    fn get_value_at(&self, indices: Ind) -> Option<T> {
        self.call(indices)
    }
}
impl<C1, C2, T: Clone + Copy> MapValD4<T> for ClosureOneOf2<C1, C2, Ind, Option<T>> {
    #[inline(always)]
    fn get_value_at(&self, indices: Ind) -> Option<T> {
        self.call(indices)
    }
}
impl<C1, C2, C3, T: Clone + Copy> MapValD4<T> for ClosureOneOf3<C1, C2, C3, Ind, Option<T>> {
    #[inline(always)]
    fn get_value_at(&self, indices: Ind) -> Option<T> {
        self.call(indices)
    }
}
impl<C1, C2, C3, C4, T: Clone + Copy> MapValD4<T>
    for ClosureOneOf4<C1, C2, C3, C4, Ind, Option<T>>
{
    #[inline(always)]
    fn get_value_at(&self, indices: Ind) -> Option<T> {
        self.call(indices)
    }
}

// non-recursive - only val
impl<T: Clone + Copy> MapValD4<T> for Box<dyn Fn(Ind) -> Option<T>> {
    #[inline(always)]
    fn get_value_at(&self, indices: Ind) -> Option<T> {
        (self)(indices)
    }
}

// ndarray
#[cfg(any(feature = "impl_all", feature = "impl_ndarray"))]
impl<T: Clone + Copy> MapValD4<T> for ndarray::Array4<T> {
    #[inline(always)]
    fn get_value_at(&self, key: Ind) -> Option<T> {
        self.get([key.0, key.1, key.2, key.3]).copied()
    }
}
