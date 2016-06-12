use machine;

#[derive(Debug)]
pub struct RET;

#[derive(Debug)]
pub struct RETI;

impl machine::Instruction for RET
{
    fn mnemonic(&self) -> String { "ret".to_owned() }
    fn operands(&self) -> Vec<machine::Operand> { vec![] }

    fn encode(&self) -> machine::EncodedInstruction {
        machine::EncodedInstruction::from(0b1001_0101_0000_1000u16)
    }
}

impl machine::Instruction for RETI
{
    fn mnemonic(&self) -> String { "reti".to_owned() }
    fn operands(&self) -> Vec<machine::Operand> { vec![] }

    fn encode(&self) -> machine::EncodedInstruction {
        machine::EncodedInstruction::from(0b1001_0101_0001_1000u16)
    }
}

