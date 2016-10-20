use {Instruction, Operand, OperandInfo, EncodedInstruction, SideEffects};
use avr::registers::{GPR8hi, IWREGS};
use {mir, regalloc};
use std;

macro_rules! define_rdi_struct {
    ($name:ident, $regclass:ident) => {
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

                let rd = Operand::Register(regalloc::Register::Virtual {
                    id: dest_reg.register_id,
                    class: &$regclass,
                });

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
    ($name:ident, $mnemonic:expr, $regclass:ident) => {
        define_rdi_struct!($name, $regclass);

        impl Instruction for $name
        {
            fn mnemonic(&self) -> String { $mnemonic.to_owned() }
            fn operands(&self) -> Vec<OperandInfo> {
                vec![
                    OperandInfo::input_output(self.rd.clone()),
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

        impl_debug_for_instruction!($name);
    }
}

/// Defines an RdI instruction which doesn'tm= modift it registers.
macro_rules! define_pure_rdi {
    ($name:ident, $mnemonic:expr, $regclass:ident) => {
        define_rdi_struct!($name, $regclass);

        impl Instruction for $name
        {
            fn mnemonic(&self) -> String { $mnemonic.to_owned() }

            fn operands(&self) -> Vec<OperandInfo> {
                vec![
                    OperandInfo::input(self.rd.clone()),
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

        impl_debug_for_instruction!($name);
    }
}

define_rdi!(ADIWRdK,  "adiw", IWREGS);

define_rdi!(SUBIRdK,  "subi", GPR8hi);
define_rdi!(SBCIRdK,  "sbci", GPR8hi);
define_rdi!(ANDIRdK,  "andi", GPR8hi);
define_rdi!(ORIRdK,   "ori",  GPR8hi);

define_pure_rdi!(CPIRdK, "cpi", GPR8hi);

