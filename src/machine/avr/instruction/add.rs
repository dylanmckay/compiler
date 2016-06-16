use {Instruction, Operand, EncodedInstruction};
use mir;

#[derive(Clone, Debug)]
pub struct ADDRdRr
{
    lhs: Operand,
    rhs: Operand,
}

impl ADDRdRr
{
    pub fn new(lhs: Operand, rhs: Operand) -> Self {
        ADDRdRr { lhs: lhs, rhs: rhs }
    }

    pub fn from_pattern(node: &mir::Node) -> Box<Instruction> {
        let _branch = node.expect_branch();
        let rd = Operand::Register(0);
        let rr = Operand::Register(0);
        Box::new(Self::new(rd, rr))
    }
}

impl Instruction for ADDRdRr
{
    fn mnemonic(&self) -> String { "add".to_owned() }
    fn operands(&self) -> Vec<Operand> {
        vec![self.lhs.clone(), self.rhs.clone()]
    }

    fn encode(&self) -> EncodedInstruction {
        unimplemented!();
    }
}

