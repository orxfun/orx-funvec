use crate::{
    impl_dim1::iterators::mapref_iter::{MapRef, MapRefIter},
    FunVecD1Ref,
};

impl<T: ?Sized, V: MapRef<T>> FunVecD1Ref<T> for V {
    type Iter<'a, I> = MapRefIter<'a, T, I, Self> where T: 'a, Self: 'a, I: Iterator<Item = usize> + 'a;

    fn ref_at(&self, index: usize) -> Option<&T> {
        self.get_ref_by_key(index)
    }

    fn ref_iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = usize> + 'a,
    {
        Self::Iter::new(self, indices)
    }
}
