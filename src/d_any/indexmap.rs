use crate::{funvec_ref::FunVecRef, funvec_val::FunVec, index::IntoIndex};
use indexmap::IndexMap;

// val
impl<const DIM: usize, T: Clone + Copy> FunVec<DIM, T> for IndexMap<[usize; DIM], T> {
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        self.get(&index.into_index()).copied()
    }
}

// ref
impl<const DIM: usize, T> FunVecRef<DIM, T> for IndexMap<[usize; DIM], T> {
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T> {
        self.get(&index.into_index())
    }
}
