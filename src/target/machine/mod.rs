use Target;
use select;

pub use self::instruction::Instruction;
pub use self::encoded_instruction::EncodedInstruction;
pub use self::operand::Operand;
pub use self::pattern::{Pattern, PatternNode, PatternOperand};

pub mod instruction;
pub mod encoded_instruction;
pub mod operand;
pub mod pattern;

pub mod avr;

/// A target.
pub trait MachineTarget : Target
{
    type OpCode;

    /// Gets the width of a pointer.
    fn pointer_width(&self) -> u16;

    /// Gets register information.
    fn register_info(&self) -> &RegisterInfo;
}

pub trait RegisterInfo
{
    /// Gets the register classes the target supports.
    fn classes(&self)
        -> &'static [&'static RegisterClass];
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Register
{
    pub name: &'static str,
    pub number: u32,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct RegisterClass
{
    pub name: &'static str,
    pub bit_width: u32,
    pub registers: &'static [&'static Register],
}

pub type Selector = select::Selector<PatternOperand>;

