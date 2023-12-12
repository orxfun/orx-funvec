use crate::{
    funvec_d3::{FunVecRefD3, Ind},
    impl_dim3::iterators::mapref_iter::{MapRefD3, MapRefD3Iter},
};

impl<T: ?Sized, V: MapRefD3<T>> FunVecRefD3<T> for V {
    type Iter<'a, I> = MapRefD3Iter<'a, T, I, Self> where T: 'a, Self: 'a, I: Iterator<Item = Ind> + 'a;

    fn ref_at(&self, i: usize, j: usize, k: usize) -> Option<&T> {
        self.get_ref_at((i, j, k))
    }

    fn ref_iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = Ind> + 'a,
    {
        Self::Iter::new(self, indices)
    }
}
