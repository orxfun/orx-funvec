use crate::{funvec_val::FunVec, index::IntoIndex};
use std::marker::PhantomData;

/// An iterator over a vector of dimension `DIM` which yields values of vector elements
/// at the positions which the index iterator `IdxIter` returns.
#[derive(derive_new::new)]
pub struct IterOverValues<'a, const DIM: usize, T, Idx, IdxIter, V: ?Sized>
where
    Idx: IntoIndex<DIM>,
    IdxIter: Iterator<Item = Idx> + 'a,
    V: FunVec<DIM, T>,
    T: Clone + Copy,
{
    value: &'a V,
    indices_iter: IdxIter,
    ph: PhantomData<&'a (T, Idx)>,
}

impl<'a, const DIM: usize, T, Idx, IdxIter, V> Iterator
    for IterOverValues<'a, DIM, T, Idx, IdxIter, V>
where
    Idx: IntoIndex<DIM>,
    IdxIter: Iterator<Item = Idx> + 'a,
    V: FunVec<DIM, T>,
    T: Clone + Copy,
{
    type Item = Option<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.indices_iter.next().map(|i| self.value.at(i))
    }
}
