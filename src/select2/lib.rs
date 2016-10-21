pub use self::permutation::*;
pub use self::adjustment::Adjustment;
pub use self::pattern::*;
pub use self::selector::Selector;
pub use self::context::Context;

pub mod permutation;
pub mod adjustment;
pub mod pattern;
pub mod selector;
pub mod context;

extern crate compiler_mir as mir;
extern crate compiler_util as util;

/// Something that can be selected.
pub trait Selectable : ::std::fmt::Debug { }

