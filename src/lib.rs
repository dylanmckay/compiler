#![feature(plugin)]
#![feature(associated_consts)]

extern crate num;
extern crate bit_vec;

/// Various utilities.
pub use self::compiler_util as util;
/// Language-agnostic types.
pub use self::compiler_lang as lang;
/// The intermediate representation.
pub use self::compiler_ir as ir;
/// The target information module.
pub use self::compiler_target as target;
/// The pass infrastructure.
pub use self::compiler_pass as pass;

#[macro_use]
extern crate compiler_util;
extern crate compiler_lang;
extern crate compiler_ir;
extern crate compiler_target;
extern crate compiler_pass;
