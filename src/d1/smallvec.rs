use crate::{funvec_ref::FunVecRef, funvec_val::FunVec, index::IntoIndex};
use smallvec::{Array, SmallVec};

const DIM: usize = 1;

impl<T: Clone + Copy, A: Array<Item = T>> FunVec<DIM, T> for SmallVec<A> {
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        self.get(index.into_index()[0]).copied()
    }
}
impl<T, A: Array<Item = T>> FunVecRef<DIM, T> for SmallVec<A> {
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T> {
        self.get(index.into_index()[0])
    }
}
