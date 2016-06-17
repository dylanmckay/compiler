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

pub mod avr;

extern crate compiler_mir as mir;
extern crate compiler_regalloc as regalloc;
extern crate compiler_select as select;
extern crate compiler_target as target;
extern crate compiler_util as util;

extern crate bit_vec;

/// A target.
pub trait MachineTarget : target::Target +
    regalloc::Target<Instruction=Box<Instruction>,
                     RegisterClass=&'static RegisterClass,
                     Register=&'static Register>
{
    /// Gets the width of a pointer.
    fn pointer_width(&self) -> u16;

    /// Gets register information.
    fn register_info(&self) -> &RegisterInfo;

    fn create_legalizer(&self) -> select::Legalizer;
    fn create_selector(&self) -> Selector;
}

pub type Selector = select::Selector<Box<Instruction>, PatternOperand>;

// TODO: this doesn't belong here, but it's good for testing.
pub fn assemble<T>(target: &T, dag: mir::Dag)
    where T: MachineTarget {
    use regalloc;

    let legalizer = target.create_legalizer();
    let mut selector = target.create_selector();

    let dag = legalizer.legalize(dag);
    let instructions = selector.select(dag);

    println!("Instruction selection: {:#?}", instructions);

    let instructions = regalloc::allocate::<T>(target, instructions);
    println!("Register allocation: {:#?}", instructions);
}

