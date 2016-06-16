use {Instruction, Operand, EncodedInstruction};
use mir;

#[derive(Clone, Debug)]
pub struct RET;

#[derive(Clone, Debug)]
pub struct RETI;

impl RET
{
    pub fn from_pattern(node: &mir::Node) -> Box<Instruction> {
        Box::new(RET)
    }
}

impl Instruction for RET
{
    fn mnemonic(&self) -> String { "ret".to_owned() }
    fn operands(&self) -> Vec<Operand> { vec![] }

    fn encode(&self) -> EncodedInstruction {
        EncodedInstruction::from(0b1001_0101_0000_1000u16)
    }
}

impl Instruction for RETI
{
    fn mnemonic(&self) -> String { "reti".to_owned() }
    fn operands(&self) -> Vec<Operand> { vec![] }

    fn encode(&self) -> EncodedInstruction {
        EncodedInstruction::from(0b1001_0101_0001_1000u16)
    }
}

