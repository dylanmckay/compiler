
#![feature(iter_arith,plugin)]
#![feature(associated_consts)]

// #![plugin(clippy)]

extern crate num;
extern crate bit_vec;

pub use self::compiler_util as util;
pub use self::compiler_lang as lang;

/// Various utilities.
#[macro_use]
extern crate compiler_util;
/// The immediate representation (IR).
pub mod ir;
/// Language-agnostic traits.
extern crate compiler_lang;
/// The pass infrastructure.
pub mod pass;
/// The target information module.
pub mod target;

