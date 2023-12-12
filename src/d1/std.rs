use crate::{funvec_ref::FunVecRef, funvec_val::FunVec, index::IntoIndex};

const DIM: usize = 1;

// val
impl<T: Clone + Copy> FunVec<DIM, T> for Vec<T> {
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        self.get(index.into_index()[0]).copied()
    }
}
impl<const N: usize, T: Clone + Copy> FunVec<DIM, T> for [T; N] {
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        self.get(index.into_index()[0]).copied()
    }
}

// ref
impl<T> FunVecRef<DIM, T> for Vec<T> {
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T> {
        self.get(index.into_index()[0])
    }
}
impl<const N: usize, T> FunVecRef<DIM, T> for [T; N] {
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T> {
        self.get(index.into_index()[0])
    }
}
