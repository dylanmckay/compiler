
#![feature(iter_arith,plugin)]
#![feature(associated_consts)]

// #![plugin(clippy)]

extern crate num;
extern crate bit_vec;

/// Various utilities.
pub use self::compiler_util as util;
/// Language-agnostic types.
pub use self::compiler_lang as lang;
/// The target information module.
pub use self::compiler_target as target;

pub mod pass;
pub mod ir;

#[macro_use]
extern crate compiler_util;
extern crate compiler_lang;
extern crate compiler_target;
