pub use self::opcodes::OpCode;
pub use self::branch::Branch;
pub use self::value::{Value, RegisterRef, ConstantInteger, ValueInfo};
pub use self::node::{Node, NodeKind};
pub use self::dag::Dag;
pub use self::ty::Type;

pub mod opcodes;
pub mod branch;
pub mod value;
pub mod node;
pub mod dag;
pub mod ty;

pub mod builder;
pub mod verifier;
pub mod expand;

extern crate num;
extern crate compiler_ir as ir;
extern crate compiler_util as util;

