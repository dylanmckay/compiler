pub use self::avr::AVR;

pub use self::instruction::{Instruction, SideEffects};
pub use self::encoded_instruction::EncodedInstruction;
pub use self::operand::{OperandInfo, Operand};
pub use self::pattern::{Pattern, PatternNode, PatternOperand};

pub mod instruction;
pub mod encoded_instruction;
pub mod operand;
pub mod pattern;

pub mod avr;

extern crate compiler_mir as mir;
extern crate compiler_select as select;
extern crate compiler_target as target;
extern crate compiler_util as util;

extern crate bit_vec;

/// A target.
pub trait MachineTarget : target::Target
{
    /// Gets the width of a pointer.
    fn pointer_width(&self) -> u16;

    /// Gets register information.
    fn register_info(&self) -> &RegisterInfo;

    fn create_legalizer(&self) -> select::Legalizer;
    fn create_selector(&self) -> Selector;
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

pub type Selector = select::Selector<Box<Instruction>, PatternOperand>;

// TODO: this doesn't belong here, but it's good for testing.
pub fn assemble<T>(target: &T, dag: mir::Dag)
    where T: MachineTarget {
    let legalizer = target.create_legalizer();
    let mut selector = target.create_selector();

    let dag = legalizer.legalize(dag);
    let patterns = selector.select(dag);

    println!("Instruction_selection: {:#?}", patterns);
}

