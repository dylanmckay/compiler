use {Instruction, Operand, OperandInfo, EncodedInstruction, SideEffects};
use avr::registers::GPR8;
use mir;
use std;

macro_rules! define_rdi_struct {
    ($name:ident) => {
        #[derive(Clone)]
        pub struct $name
        {
            pub rd: Operand,
            pub i: Operand,
        }

        impl $name
        {
            pub fn new(rd: Operand, i: Operand) -> Self {
                $name { rd: rd, i: i }
            }

            pub fn from_pattern(node: &mir::Node) -> Box<Instruction> {
                let set = node.expect_branch();
                let dest_reg = set.operands[0].expect_leaf().expect_register_ref();
                let value = set.operands[1].expect_branch();
                let imm = value.operands[1].expect_leaf().
                    expect_constant_integer();

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
    }
}

macro_rules! define_rdi {
    ($name:ident, $mnemonic:expr) => {
        define_rdi_struct!($name);

        impl Instruction for $name
        {
            fn mnemonic(&self) -> String { $mnemonic.to_owned() }
            fn operands(&self) -> Vec<OperandInfo> {
                vec![
                    OperandInfo::input_output(self.rd.clone()),
                    OperandInfo::input(self.i.clone()),
                ]
            }

            fn side_effects(&self) -> SideEffects {
                SideEffects::none()
            }

            fn encode(&self) -> EncodedInstruction {
                unimplemented!();
            }
        }

        impl_debug_for_instruction!($name);
    }
}

/// Defines an RdI instruction which doesn'tm= modift it registers.
macro_rules! define_pure_rdi {
    ($name:ident, $mnemonic:expr) => {
        define_rdi_struct!($name);

        impl Instruction for $name
        {
            fn mnemonic(&self) -> String { $mnemonic.to_owned() }
            fn operands(&self) -> Vec<OperandInfo> {
                vec![
                    OperandInfo::input(self.rd.clone()),
                    OperandInfo::input(self.i.clone()),
                ]
            }

            fn side_effects(&self) -> SideEffects {
                SideEffects::none()
            }

            fn encode(&self) -> EncodedInstruction {
                unimplemented!();
            }
        }

        impl_debug_for_instruction!($name);
    }
}

define_rdi!(ADIWRdK,  "adiw");
define_rdi!(SUBIRdK,  "subi");
define_rdi!(SBCIRdK,  "sbci");
define_rdi!(ANDIRdK,  "andi");
define_rdi!(ORIRdK,   "ori");
define_pure_rdi!(CPIRdK,   "cpi");

