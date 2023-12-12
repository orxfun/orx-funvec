use crate::index::{FromIndex, IntoIndex};

const DIM: usize = 1;
type Tuple = usize;

impl IntoIndex<DIM> for Tuple {
    #[inline(always)]
    fn into_index(self) -> [usize; DIM] {
        [self]
    }
}

impl FromIndex<DIM> for Tuple {
    #[inline(always)]
    fn from_index(index: [usize; DIM]) -> Self {
        index[0]
    }
}
