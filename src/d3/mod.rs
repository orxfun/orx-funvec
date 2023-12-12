mod into_index;
mod std;

#[cfg(any(feature = "impl_all", feature = "impl_indexmap"))]
mod indexmap;

#[cfg(any(feature = "impl_all", feature = "impl_ndarray"))]
mod ndarray;

#[cfg(any(feature = "impl_all", feature = "impl_smallvec"))]
mod smallvec;
