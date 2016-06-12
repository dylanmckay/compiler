use machine;

#[derive(Debug)]
pub struct RET;

impl machine::Instruction for RET
{
    fn mnemonic(&self) -> String { "ret".to_owned() }
    fn operands(&self) -> Vec<machine::Operand> { vec![] }
}

