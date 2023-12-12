use crate::{funvec_val::FunVec, index::IntoIndex};
use smallvec::{Array, SmallVec};

const DIM: usize = 2;
const LOW_DIM: usize = DIM - 1;

impl<T: Clone + Copy, V1: FunVec<LOW_DIM, T>, A: Array<Item = V1>> FunVec<DIM, T> for SmallVec<A> {
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        let [i, j] = index.into_index();
        self.get(i).and_then(|x| x.at([j]))
    }
}
