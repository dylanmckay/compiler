use {Instruction, Operand, OperandInfo, EncodedInstruction, SideEffects};
use avr::registers::GPR8hi;
use {mir, regalloc};
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

        let rd = Operand::Regalloc(regalloc::Operand::VirtualRegister {
            id: dest_reg.register_id,
            class: &GPR8hi,
        });

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
    fn operands(&self) -> Vec<OperandInfo> {
        vec![
            OperandInfo::output(self.rd.clone()),
            OperandInfo::input(self.i.clone()),
        ]
    }

    fn operands_mut(&mut self) -> Vec<&mut Operand> {
        vec![&mut self.rd, &mut self.i]
    }

    fn side_effects(&self) -> SideEffects {
        SideEffects::none()
    }

    fn encode(&self) -> EncodedInstruction {
        unimplemented!();
    }
}

impl_debug_for_instruction!(LDIRdK);

