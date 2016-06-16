use {Instruction, Operand, EncodedInstruction};
use avr::registers::GPR8;
use mir;
use std;

#[derive(Clone)]
pub struct MOVRdRr
{
    pub rd: Operand,
    pub rr: Operand,
}

impl MOVRdRr
{
    pub fn new(rd: Operand, rr: Operand) -> Self {
        MOVRdRr { rd: rd, rr: rr }
    }

    pub fn from_pattern(node: &mir::Node) -> Box<Instruction> {
        let set = node.expect_branch();
        let dest_reg = set.operands[0].expect_leaf().expect_register_ref();
        let source_reg = set.operands[1].expect_leaf().expect_register_ref();

        let rd = Operand::VirtualRegister { id: dest_reg.register_id, class: &GPR8 };
        let rr = Operand::VirtualRegister { id: source_reg.register_id, class: &GPR8 };

        Box::new(Self::new(rd, rr))
    }
}

impl Instruction for MOVRdRr
{
    fn mnemonic(&self) -> String { "mov".to_owned() }
    fn operands(&self) -> Vec<Operand> {
        vec![self.rd.clone(), self.rr.clone()]
    }

    fn encode(&self) -> EncodedInstruction {
        unimplemented!();
    }
}

impl std::fmt::Debug for MOVRdRr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        try!(write!(fmt, "{} ", self.mnemonic()));

        let operands: Vec<_> = self.operands().iter().map(|op| format!("{:?}", op)).collect();
        try!(write!(fmt, "{}", operands.join(", ")));

        Ok(())
    }
}

