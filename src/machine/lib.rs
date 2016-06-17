pub use self::avr::AVR;

pub use self::instruction::{Instruction, SideEffects};
pub use self::encoded_instruction::EncodedInstruction;
pub use self::operand::{OperandInfo, Operand, Direction};
pub use self::pattern::{Pattern, PatternNode, PatternOperand};
pub use self::register::{RegisterInfo, Register, RegisterClass};

pub mod instruction;
pub mod encoded_instruction;
pub mod operand;
pub mod pattern;
pub mod register;
pub mod generate;

pub mod avr;

extern crate compiler_ir as ir;
extern crate compiler_mir as mir;
extern crate compiler_regalloc as regalloc;
extern crate compiler_select as select;
extern crate compiler_target as target;
extern crate compiler_util as util;

extern crate bit_vec;

/// A target.
pub trait MachineTarget : target::Target + regalloc::Target<Instruction=Box<Instruction>,
                                                            Operand=Operand,
                                                            RegisterClass=&'static RegisterClass,
                                                            Register=&'static Register>
{
    /// Gets the width of a pointer.
    fn pointer_width(&self) -> u16;

    fn create_legalizer(&self) -> select::Legalizer;
    fn create_selector(&self) -> Selector;
}

pub type Selector = select::Selector<Box<Instruction>, PatternOperand>;

