use crate::{funvec_ref::FunVecRef, funvec_val::FunVec, index::IntoIndex};
use std::collections::{BTreeMap, HashMap};

const DIM: usize = 3;
const LOW_DIM: usize = DIM - 1;

// val
impl<T: Clone + Copy, V1: FunVec<LOW_DIM, T>> FunVec<DIM, T> for Vec<V1> {
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        let [i, j, k] = index.into_index();
        self.get(i).and_then(|x| x.at([j, k]))
    }
}
impl<const N: usize, T: Clone + Copy, V1: FunVec<LOW_DIM, T>> FunVec<DIM, T> for [V1; N] {
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        let [i, j, k] = index.into_index();
        self.get(i).and_then(|x| x.at([j, k]))
    }
}

impl<T: Clone + Copy, V1: FunVec<LOW_DIM, T>> FunVec<DIM, T> for HashMap<usize, V1> {
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        let [i, j, k] = index.into_index();
        self.get(&i).and_then(|x| x.at([j, k]))
    }
}
impl<T: Clone + Copy, V1: FunVec<LOW_DIM, T>> FunVec<DIM, T> for BTreeMap<usize, V1> {
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        let [i, j, k] = index.into_index();
        self.get(&i).and_then(|x| x.at([j, k]))
    }
}

// ref
impl<T: Clone + Copy, V1: FunVecRef<LOW_DIM, T>> FunVecRef<DIM, T> for Vec<V1> {
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T> {
        let [i, j, k] = index.into_index();
        self.get(i).and_then(|x| x.ref_at([j, k]))
    }
}
impl<const N: usize, T, V1: FunVecRef<LOW_DIM, T>> FunVecRef<DIM, T> for [V1; N] {
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T> {
        let [i, j, k] = index.into_index();
        self.get(i).and_then(|x| x.ref_at([j, k]))
    }
}

impl<T, V1: FunVecRef<LOW_DIM, T>> FunVecRef<DIM, T> for HashMap<usize, V1> {
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T> {
        let [i, j, k] = index.into_index();
        self.get(&i).and_then(|x| x.ref_at([j, k]))
    }
}
impl<T, V1: FunVecRef<LOW_DIM, T>> FunVecRef<DIM, T> for BTreeMap<usize, V1> {
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T> {
        let [i, j, k] = index.into_index();
        self.get(&i).and_then(|x| x.ref_at([j, k]))
    }
}
