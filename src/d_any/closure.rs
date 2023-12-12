use crate::{
    funvec_ref::FunVecRef,
    funvec_val::FunVec,
    index::{FromIndex, IntoIndex},
};
use orx_closure::{
    Closure, ClosureOneOf2, ClosureOneOf3, ClosureOneOf4, ClosureOptRef, ClosureOptRefOneOf2,
    ClosureOptRefOneOf3, ClosureOptRefOneOf4,
};

// val
impl<const DIM: usize, C1, In: FromIndex<DIM>, T: Clone + Copy> FunVec<DIM, T>
    for Closure<C1, In, Option<T>>
{
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        let index = In::from_index(index.into_index());
        self.call(index)
    }
}
impl<const DIM: usize, C1, C2, In: FromIndex<DIM>, T: Clone + Copy> FunVec<DIM, T>
    for ClosureOneOf2<C1, C2, In, Option<T>>
{
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        let index = In::from_index(index.into_index());
        self.call(index)
    }
}
impl<const DIM: usize, C1, C2, C3, In: FromIndex<DIM>, T: Clone + Copy> FunVec<DIM, T>
    for ClosureOneOf3<C1, C2, C3, In, Option<T>>
{
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        let index = In::from_index(index.into_index());
        self.call(index)
    }
}
impl<const DIM: usize, C1, C2, C3, C4, In: FromIndex<DIM>, T: Clone + Copy> FunVec<DIM, T>
    for ClosureOneOf4<C1, C2, C3, C4, In, Option<T>>
{
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        let index = In::from_index(index.into_index());
        self.call(index)
    }
}

// ref
impl<const DIM: usize, C1, In: FromIndex<DIM>, T: ?Sized> FunVecRef<DIM, T>
    for ClosureOptRef<C1, In, T>
{
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T> {
        let index = In::from_index(index.into_index());
        self.call(index)
    }
}
impl<const DIM: usize, C1, C2, In: FromIndex<DIM>, T: ?Sized> FunVecRef<DIM, T>
    for ClosureOptRefOneOf2<C1, C2, In, T>
{
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T> {
        let index = In::from_index(index.into_index());
        self.call(index)
    }
}
impl<const DIM: usize, C1, C2, C3, In: FromIndex<DIM>, T: ?Sized> FunVecRef<DIM, T>
    for ClosureOptRefOneOf3<C1, C2, C3, In, T>
{
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T> {
        let index = In::from_index(index.into_index());
        self.call(index)
    }
}
impl<const DIM: usize, C1, C2, C3, C4, In: FromIndex<DIM>, T: ?Sized> FunVecRef<DIM, T>
    for ClosureOptRefOneOf4<C1, C2, C3, C4, In, T>
{
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T> {
        let index = In::from_index(index.into_index());
        self.call(index)
    }
}
