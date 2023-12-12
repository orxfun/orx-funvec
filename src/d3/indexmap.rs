use crate::{funvec_val::FunVec, index::IntoIndex, FunVecRef};
use indexmap::IndexMap;

const DIM: usize = 3;
const LOW_DIM: usize = DIM - 1;

// val
impl<T: Clone + Copy, V1: FunVec<LOW_DIM, T>> FunVec<DIM, T> for IndexMap<usize, V1> {
    #[inline(always)]
    fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T> {
        let [i, j, k] = index.into_index();
        self.get(&i).and_then(|x| x.at([j, k]))
    }
}

// ref
impl<T, V1: FunVecRef<LOW_DIM, T>> FunVecRef<DIM, T> for IndexMap<usize, V1> {
    #[inline(always)]
    fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T> {
        let [i, j, k] = index.into_index();
        self.get(&i).and_then(|x| x.ref_at([j, k]))
    }
}
