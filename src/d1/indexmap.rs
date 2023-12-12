use crate::{funvec_ref::FunVecRef, funvec_val::FunVec, index::IntoIndex};
use indexmap::IndexMap;

const DIM: usize = 1;

// val
impl<T: Clone + Copy> FunVec<DIM, T> for IndexMap<usize, T> {
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        self.get(&index.into_index()[0]).copied()
    }
}

// ref
impl<T> FunVecRef<DIM, T> for IndexMap<usize, T> {
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T> {
        self.get(&index.into_index()[0])
    }
}
