use crate::{
    funvec_d2::{FunVecRefD2, Ind},
    impl_dim2::iterators::mapref_iter::{MapRefD2, MapRefD2Iter},
};

impl<T: ?Sized, V: MapRefD2<T>> FunVecRefD2<T> for V {
    type Iter<'a, I> = MapRefD2Iter<'a, T, I, Self> where T: 'a, Self: 'a, I: Iterator<Item = Ind> + 'a;

    fn ref_at(&self, i: usize, j: usize) -> Option<&T> {
        self.get_ref_at((i, j))
    }

    fn ref_iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = Ind> + 'a,
    {
        Self::Iter::new(self, indices)
    }
}
