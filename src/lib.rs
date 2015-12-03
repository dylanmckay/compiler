
#![feature(iter_arith,plugin)]
#![feature(associated_consts)]

// #![plugin(clippy)]

extern crate num;
extern crate bit_vec;

/// Various utilities.
#[macro_use]
pub mod util;
/// The immediate representation (IR).
pub mod ir;
/// Language-agnostic traits.
pub mod lang;
/// The pass infrastructure.
pub mod pass;
/// The target information module.
pub mod target;

