
pub use self::name::Name;
pub use self::module::Module;
pub use self::global::Global;
pub use self::function::{Function,Signature,Parameter};
pub use self::block::Block;
pub use self::value::Value;
pub use self::ty::Type;
pub use self::attrs::{CallingConvention,InlineHint,ComplexityHint};

#[macro_use]
extern crate compiler_util as util;

pub mod name;
pub mod module;
pub mod global;
pub mod function;
pub mod block;
pub mod value;
pub mod ty;
pub mod attrs;

