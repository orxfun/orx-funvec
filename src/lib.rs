#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]

mod d1;
mod d2;
mod d3;
mod d4;
mod d_any;
mod empty_vec;
mod funvec_ref;
mod funvec_val;
mod index;
mod iter_over_ref;
mod iter_over_val;
mod scalar_as_vec;

pub use empty_vec::EmptyVec;
pub use funvec_ref::FunVecRef;
pub use funvec_val::FunVec;
pub use index::{FromIndex, IntoIndex};
pub use scalar_as_vec::ScalarAsVec;
