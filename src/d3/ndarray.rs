use crate::{funvec_ref::FunVecRef, funvec_val::FunVec, index::IntoIndex};
use ndarray::Array3;

const DIM: usize = 3;

impl<T: Clone + Copy> FunVec<DIM, T> for Array3<T> {
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        self.get(index.into_index()).copied()
    }
}
impl<T> FunVecRef<DIM, T> for Array3<T> {
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T> {
        self.get(index.into_index())
    }
}
