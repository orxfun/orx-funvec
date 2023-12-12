use crate::{
    funvec_d4::{FunVecRefD4, Ind},
    impl_dim4::iterators::mapref_iter::{MapRefD4, MapRefD4Iter},
};

impl<T: ?Sized, V: MapRefD4<T>> FunVecRefD4<T> for V {
    type Iter<'a, I> = MapRefD4Iter<'a, T, I, Self> where T: 'a, Self: 'a, I: Iterator<Item = Ind> + 'a;

    fn ref_at(&self, i: usize, j: usize, k: usize, l: usize) -> Option<&T> {
        self.get_ref_at((i, j, k, l))
    }

    fn ref_iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = Ind> + 'a,
    {
        Self::Iter::new(self, indices)
    }
}
