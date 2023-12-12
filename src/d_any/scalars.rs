use crate::{
    empty_vec::EmptyVec, funvec_ref::FunVecRef, funvec_val::FunVec, index::IntoIndex,
    scalar_as_vec::ScalarAsVec,
};

// val
impl<const DIM: usize, T: Clone + Copy> FunVec<DIM, T> for ScalarAsVec<T> {
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, _: Idx) -> Option<T> {
        Some(self.0)
    }
}

impl<const DIM: usize, T: Clone + Copy> FunVec<DIM, T> for EmptyVec<T> {
    fn at<Idx: IntoIndex<DIM>>(&self, _: Idx) -> Option<T> {
        None
    }
}

// ref
impl<const DIM: usize, T> FunVecRef<DIM, T> for ScalarAsVec<T> {
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, _: Idx) -> Option<&T> {
        Some(&self.0)
    }
}

impl<const DIM: usize, T: ?Sized> FunVecRef<DIM, T> for EmptyVec<T> {
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, _: Idx) -> Option<&T> {
        None
    }
}
