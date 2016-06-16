use {Instruction, Operand, OperandInfo, EncodedInstruction, SideEffects};
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
    fn operands(&self) -> Vec<OperandInfo> {
        vec![
            OperandInfo::output(self.rd.clone()),
            OperandInfo::input(self.rr.clone()),
        ]
    }

    fn side_effects(&self) -> SideEffects {
        SideEffects::none()
    }

    fn encode(&self) -> EncodedInstruction {
        unimplemented!();
    }
}

impl_debug_for_instruction!(MOVRdRr);

