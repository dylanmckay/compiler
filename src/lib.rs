#![feature(plugin)]
#![feature(associated_consts)]

#![plugin(clippy)]

extern crate num;
extern crate bit_vec;

/// Various utilities.
pub use self::compiler_util as util;
/// Language-agnostic types.
pub use self::compiler_lang as lang;
/// The intermediate representation.
pub use self::compiler_ir as ir;
/// The instruction selector.
pub use self::compiler_isel as isel;
/// The target information module.
pub use self::compiler_target as target;
/// The pass infrastructure.
pub use self::compiler_pass as pass;
/// The integrated tester.
pub use self::compiler_test as test;

#[macro_use]
pub extern crate compiler_util;
pub extern crate compiler_lang;
pub extern crate compiler_ir;
pub extern crate compiler_isel;
pub extern crate compiler_target;
pub extern crate compiler_pass;
pub extern crate compiler_test;
