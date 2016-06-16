use {Operand, EncodedInstruction};
use std;

/// A generic machine instruction.
pub trait Instruction : std::fmt::Debug
{
    fn mnemonic(&self) -> String;
    fn operands(&self) -> Vec<Operand>;

    fn encode(&self) -> EncodedInstruction;
}

