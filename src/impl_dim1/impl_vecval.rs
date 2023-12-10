use crate::{
    funvec::FunVecD1,
    impl_dim1::iterators::mapval_iter::{MapVal, MapValIter},
};

impl<T: Clone + Copy, V: MapVal<T>> FunVecD1<T> for V {
    type Iter<'a, I> = MapValIter<'a, T, I, Self> where T: 'a, Self: 'a, I: Iterator<Item = usize> + 'a;

    fn val_at(&self, index: usize) -> Option<T> {
        self.get_val_by_key(index)
    }

    fn val_iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = usize> + 'a,
    {
        Self::Iter::new(self, indices)
    }
}
