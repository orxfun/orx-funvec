use super::iterators::mapval_iter::MapValD4Iter;
use crate::{
    funvec_d4::{FunVecD4, Ind},
    impl_dim4::iterators::mapval_iter::MapValD4,
};

impl<T: Clone + Copy, V: MapValD4<T>> FunVecD4<T> for V {
    type Iter<'a, I> = MapValD4Iter<'a, T, I, Self> where T: 'a, Self: 'a, I: Iterator<Item = Ind> + 'a;

    fn val_at(&self, i: usize, j: usize, k: usize, l: usize) -> Option<T> {
        self.get_val_by_key((i, j, k, l))
    }

    fn val_iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = Ind> + 'a,
    {
        Self::Iter::new(self, indices)
    }
}
