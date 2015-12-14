pub use self::target::{RegisterClass,Target,MachineTarget};
pub use self::value::Value;
pub use self::ty::Type;

pub mod value;
pub mod ty;

extern crate compiler_lang as lang;
extern crate compiler_ir as ir;

pub type Module = ::lang::Module<Value>;
pub type Global = ::lang::Global<Value>;
pub type Function = ::lang::Function<Value>;
pub type Block = ::lang::Block<Value>;
pub type Signature = ::lang::Signature<Value>;
pub type Parameter = ::lang::Parameter<Value>;

pub mod target
{
    use std;

    #[derive(Clone,Debug,PartialEq,Eq)]
    pub struct RegisterClass
    {
        name: String,
        size: u16,
    }

    /// A target.
    pub trait Target
    {
        fn name(&self) -> &'static str;
    }

    /// A target.
    pub trait MachineTarget : Target
    {
        /// Gets the width of a pointer.
        fn pointer_width(&self) -> u16;

        /// Gets the register classes the target supports.
        fn register_classes(&self) -> std::slice::Iter<RegisterClass>;
    }
}

