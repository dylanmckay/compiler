pub use self::machine::{RegisterClass,MachineTarget};
pub use self::opcodes::OpCode;

pub mod machine;
pub mod opcodes;

extern crate compiler_ir as ir;

/// A target.
pub trait Target
{
    fn name(&self) -> &'static str;
}

