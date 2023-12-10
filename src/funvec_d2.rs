pub(crate) type Ind = (usize, usize);

pub trait FunVecD2<T> {
    type Iter<'a, I>: Iterator<Item = Option<T>> + 'a
    where
        T: 'a,
        Self: 'a,
        I: Iterator<Item = Ind> + 'a;

    fn val_at(&self, i: usize, j: usize) -> Option<T>;

    fn val_iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = Ind> + 'a;
}

pub trait FunVecRefD2<T: ?Sized> {
    type Iter<'a, I>: Iterator<Item = Option<&'a T>> + 'a
    where
        T: 'a,
        Self: 'a,
        I: Iterator<Item = Ind> + 'a;

    fn ref_at(&self, i: usize, j: usize) -> Option<&T>;

    fn ref_iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = Ind> + 'a;
}
