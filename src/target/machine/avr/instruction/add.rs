use machine::{self, Operand};

#[derive(Debug)]
pub struct ADD
{
    lhs: Operand,
    rhs: Operand,
}

impl ADD
{
    fn new(lhs: Operand, rhs: Operand) -> Self {
        ADD { lhs: lhs, rhs: rhs }
    }
}

impl machine::Instruction for ADD
{
    fn mnemonic(&self) -> String { "add".to_owned() }
    fn operands(&self) -> Vec<Operand> {
        vec![self.lhs.clone(), self.rhs.clone()]
    }

    fn encode(&self) -> machine::EncodedInstruction {
        unimplemented!();
    }
}

