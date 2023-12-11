use super::iterators::mapval_iter::MapValD3Iter;
use crate::{
    funvec_d3::{FunVecD3, Ind},
    impl_dim3::iterators::mapval_iter::MapValD3,
};

impl<T: Clone + Copy, V: MapValD3<T>> FunVecD3<T> for V {
    type Iter<'a, I> = MapValD3Iter<'a, T, I, Self> where T: 'a, Self: 'a, I: Iterator<Item = Ind> + 'a;

    fn at(&self, i: usize, j: usize, k: usize) -> Option<T> {
        self.get_val_by_key((i, j, k))
    }

    fn iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = Ind> + 'a,
    {
        Self::Iter::new(self, indices)
    }
}
