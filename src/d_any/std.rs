use crate::{
    funvec_ref::FunVecRef,
    funvec_val::FunVec,
    index::{FromIndex, IntoIndex},
};
use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

// val
impl<const DIM: usize, Key, T> FunVec<DIM, T> for HashMap<Key, T>
where
    Key: FromIndex<DIM> + PartialEq + Eq + Hash,
    T: Clone + Copy,
{
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        let index = Key::from_index(index.into_index());
        self.get(&index).copied()
    }
}
impl<const DIM: usize, Key, T> FunVec<DIM, T> for BTreeMap<Key, T>
where
    Key: FromIndex<DIM> + Ord,
    T: Clone + Copy,
{
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        let index = Key::from_index(index.into_index());
        self.get(&index).copied()
    }
}

// ref
impl<const DIM: usize, Key, T> FunVecRef<DIM, T> for HashMap<Key, T>
where
    Key: FromIndex<DIM> + PartialEq + Eq + Hash,
{
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T> {
        let index = Key::from_index(index.into_index());
        self.get(&index)
    }
}
impl<const DIM: usize, Key, T> FunVecRef<DIM, T> for BTreeMap<Key, T>
where
    Key: FromIndex<DIM> + Ord,
{
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T> {
        let index = Key::from_index(index.into_index());
        self.get(&index)
    }
}
