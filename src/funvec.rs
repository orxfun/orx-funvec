pub trait FunVecD1<T> {
    type Iter<'a, I>: Iterator<Item = Option<T>> + 'a
    where
        T: 'a,
        Self: 'a,
        I: Iterator<Item = usize> + 'a;

    fn val_at(&self, index: usize) -> Option<T>;

    fn val_iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = usize> + 'a;
}

pub trait FunVecD1Ref<T: ?Sized> {
    type Iter<'a, I>: Iterator<Item = Option<&'a T>> + 'a
    where
        T: 'a,
        Self: 'a,
        I: Iterator<Item = usize> + 'a;

    fn ref_at(&self, index: usize) -> Option<&T>;

    fn ref_iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = usize> + 'a;
}
