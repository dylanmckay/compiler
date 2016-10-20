pub use self::adjustment::Adjustment;
pub use self::pattern::*;

pub mod permutation;
pub mod adjustment;
pub mod pattern;

extern crate compiler_mir as mir;
extern crate compiler_util as util;

/// Something that can be selected.
pub trait Selectable : ::std::fmt::Debug { }

