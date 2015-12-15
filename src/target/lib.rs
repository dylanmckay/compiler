pub use self::value::Value;
pub use self::ty::Type;
pub use self::machine::{RegisterClass,MachineTarget};
pub use self::opcodes::OpCode;

pub mod value;
pub mod ty;
pub mod machine;
pub mod opcodes;

extern crate compiler_lang as lang;
extern crate compiler_ir as ir;

pub type Module = ::lang::Module<Value>;
pub type Global = ::lang::Global<Value>;
pub type Function = ::lang::Function<Value>;
pub type Block = ::lang::Block<Value>;
pub type Signature = ::lang::Signature<Value>;
pub type Parameter = ::lang::Parameter<Value>;

/// A target.
pub trait Target
{
    fn name(&self) -> &'static str;
}

