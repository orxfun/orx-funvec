use crate::{
    funvec_val::FunVec,
    index::{FromIndex, IntoIndex},
};

// val
impl<const DIM: usize, In: FromIndex<DIM>, T: Clone + Copy> FunVec<DIM, T>
    for Box<dyn Fn(In) -> Option<T>>
{
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        let index = In::from_index(index.into_index());
        (self)(index)
    }
}
