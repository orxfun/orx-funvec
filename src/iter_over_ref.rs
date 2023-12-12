use crate::{funvec_ref::FunVecRef, index::IntoIndex};
use std::marker::PhantomData;

/// An iterator over a vector of dimension `DIM` which yields references to vector elements
/// at the positions which the index iterator `IdxIter` returns.
#[derive(derive_new::new)]
pub struct IterOverRefs<'a, const DIM: usize, T, Idx, IdxIter, V: ?Sized>
where
    Idx: IntoIndex<DIM>,
    IdxIter: Iterator<Item = Idx> + 'a,
    V: FunVecRef<DIM, T>,
    T: ?Sized,
{
    value: &'a V,
    indices_iter: IdxIter,
    ph: PhantomData<&'a (&'a T, Idx)>,
}

impl<'a, const DIM: usize, T, Idx, IdxIter, V> Iterator
    for IterOverRefs<'a, DIM, T, Idx, IdxIter, V>
where
    Idx: IntoIndex<DIM>,
    IdxIter: Iterator<Item = Idx> + 'a,
    V: FunVecRef<DIM, T>,
    T: ?Sized,
{
    type Item = Option<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.indices_iter.next().map(|i| self.value.ref_at(i))
    }
}
