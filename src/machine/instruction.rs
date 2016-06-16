use {Operand, EncodedInstruction, Register};
use std;

/// A generic machine instruction.
pub trait Instruction : std::fmt::Debug
{
    /// Gets the mnemonic of the instruction.
    fn mnemonic(&self) -> String;
    fn operands(&self) -> Vec<Operand>;

    /// Gets the side effects of the instruction.
    fn side_effects(&self) -> SideEffects;

    fn encode(&self) -> EncodedInstruction;
}

/// Keeps track of the side effects of an instruction.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct SideEffects
{
    uses: Vec<&'static Register>,
    defs: Vec<&'static Register>,
}

impl SideEffects
{
    pub fn none() -> Self {
        SideEffects {
            uses: Vec::new(),
            defs: Vec::new(),
        }
    }

    /// Marks a register as used.
    pub fn uses(mut self, register: &'static Register) -> Self {
        self.uses.push(register);
        self
    }

    /// Marks a register as defined.
    pub fn defs(mut self, register: &'static Register) -> Self {
        self.defs.push(register);
        self
    }
}

