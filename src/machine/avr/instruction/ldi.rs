use {Instruction, Operand, EncodedInstruction, SideEffects};
use avr::registers::GPR8;
use mir;
use std;

#[derive(Clone)]
pub struct LDIRdK
{
    pub rd: Operand,
    pub i: Operand,
}

impl LDIRdK
{
    pub fn new(rd: Operand, i: Operand) -> Self {
        LDIRdK { rd: rd, i: i }
    }

    pub fn from_pattern(node: &mir::Node) -> Box<Instruction> {
        let set = node.expect_branch();
        let dest_reg = set.operands[0].expect_leaf().expect_register_ref();
        let imm = set.operands[1].expect_leaf().expect_constant_integer();

        let rd = Operand::VirtualRegister {
            id: dest_reg.register_id,
            class: &GPR8,
        };

        let i = Operand::Immediate {
            bit_width: imm.bit_width,
            value: imm.value,
        };

        Box::new(Self::new(rd, i))
    }
}

impl Instruction for LDIRdK
{
    fn mnemonic(&self) -> String { "ldi".to_owned() }
    fn operands(&self) -> Vec<Operand> {
        vec![self.rd.clone(), self.i.clone()]
    }

    fn side_effects(&self) -> SideEffects {
        SideEffects::none()
    }

    fn encode(&self) -> EncodedInstruction {
        unimplemented!();
    }
}

impl std::fmt::Debug for LDIRdK {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        try!(write!(fmt, "{} ", self.mnemonic()));

        let operands: Vec<_> = self.operands().iter().map(|op| format!("{:?}", op)).collect();
        try!(write!(fmt, "{}", operands.join(", ")));

        Ok(())
    }
}
