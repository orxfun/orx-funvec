pub(crate) type Ind = (usize, usize, usize, usize);

pub trait FunVecD4<T> {
    type Iter<'a, I>: Iterator<Item = Option<T>> + 'a
    where
        T: 'a,
        Self: 'a,
        I: Iterator<Item = Ind> + 'a;

    fn at(&self, i: usize, j: usize, k: usize, l: usize) -> Option<T>;

    fn iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = Ind> + 'a;
}

pub trait FunVecRefD4<T: ?Sized> {
    type Iter<'a, I>: Iterator<Item = Option<&'a T>> + 'a
    where
        T: 'a,
        Self: 'a,
        I: Iterator<Item = Ind> + 'a;

    fn ref_at(&self, i: usize, j: usize, k: usize, l: usize) -> Option<&T>;

    fn ref_iter_over<'a, I>(&self, indices: I) -> Self::Iter<'_, I>
    where
        I: Iterator<Item = Ind> + 'a;
}
