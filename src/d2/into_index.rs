use crate::index::{FromIndex, IntoIndex};

const DIM: usize = 2;
type Tuple = (usize, usize);

impl IntoIndex<DIM> for Tuple {
    #[inline(always)]
    fn into_index(self) -> [usize; DIM] {
        [self.0, self.1]
    }
}

impl FromIndex<DIM> for Tuple {
    #[inline(always)]
    fn from_index(index: [usize; DIM]) -> Self {
        (index[0], index[1])
    }
}
