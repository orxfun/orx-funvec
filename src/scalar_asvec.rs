pub struct Scalar<T>(pub(crate) Option<T>);

impl<T> Scalar<T> {
    pub fn new(value: T) -> Self {
        Self(Some(value))
    }
    pub fn none() -> Self {
        Self(None)
    }
}

impl<T> From<T> for Scalar<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
