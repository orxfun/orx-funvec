use super::iterators::mapval_iter::MapValD2Iter;
use crate::{
    funvec_d2::{FunVecD2, Ind},
    impl_dim2::iterators::mapval_iter::MapValD2,
};

impl<T: Clone + Copy, V: MapValD2<T>> FunVecD2<T> for V {
    type Iter<'a, I> = MapValD2Iter<'a, T, I, Self> where T: 'a, Self: 'a, I: Iterator<Item = Ind> + 'a;

    fn val_at(&self, i: usize, j: usize) -> Option<T> {
        self.get_val_by_key((i, j))
    }

    fn val_iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = Ind> + 'a,
    {
        Self::Iter::new(self, indices)
    }
}
