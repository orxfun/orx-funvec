use crate::{
    funvec_d1::FunVecD1,
    impl_dim1::iterators::mapval_iter::{MapVal, MapValIter},
};

impl<T: Clone + Copy, V: MapVal<T>> FunVecD1<T> for V {
    type Iter<'a, I> = MapValIter<'a, T, I, Self> where T: 'a, Self: 'a, I: Iterator<Item = usize> + 'a;

    fn at(&self, index: usize) -> Option<T> {
        self.get_value_at(index)
    }

    fn iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = usize> + 'a,
    {
        Self::Iter::new(self, indices)
    }
}
