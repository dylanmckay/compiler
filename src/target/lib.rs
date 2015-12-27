pub use self::machine::{RegisterClass,MachineTarget};
pub use self::opcodes::OpCode;
pub use self::node::Node;
pub use self::dag::Dag;

pub mod machine;
pub mod opcodes;
pub mod node;
pub mod dag;

extern crate num;
extern crate compiler_ir as ir;

/// A target.
pub trait Target
{
    fn name(&self) -> &'static str;
}

