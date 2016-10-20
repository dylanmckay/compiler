use {Instruction, Operand, OperandInfo, EncodedInstruction, SideEffects};
use avr::registers::GPR8;
use {mir, regalloc};
use std;

macro_rules! define_rdrr_struct {
    ($name:ident) => {
        #[derive(Clone)]
        pub struct $name
        {
            pub lhs: Operand,
            pub rhs: Operand,
        }

        impl $name
        {
            pub fn new(lhs: Operand, rhs: Operand) -> Self {
                $name { lhs: lhs, rhs: rhs }
            }

            pub fn from_pattern(node: &mir::Node) -> Box<Instruction> {
                let set = node.expect_branch();
                let dest_reg = set.operands[0].expect_leaf().expect_register_ref();
                let value = set.operands[1].expect_branch();
                let source_reg = value.operands[1].expect_leaf().expect_register_ref();

                let rd = Operand::Regalloc(regalloc::Operand::VirtualRegister { id: dest_reg.register_id, class: &GPR8 });
                let rr = Operand::Regalloc(regalloc::Operand::VirtualRegister { id: source_reg.register_id, class: &GPR8 });

                Box::new(Self::new(rd, rr))
            }
        }
    }
}

macro_rules! define_rdrr {
    ($name:ident, $mnemonic:expr) => {
        define_rdrr_struct!($name);

        impl Instruction for $name
        {
            fn mnemonic(&self) -> String { $mnemonic.to_owned() }

            fn operands(&self) -> Vec<OperandInfo> {
                vec![
                    OperandInfo::input_output(self.lhs.clone()),
                    OperandInfo::input(self.rhs.clone()),
                ]
            }

            fn operands_mut(&mut self) -> Vec<&mut Operand> {
                vec![&mut self.lhs, &mut self.rhs]
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

/// Defines an RDRR instruction which doesn't modify and registers.
macro_rules! define_pure_rdrr {
    ($name:ident, $mnemonic:expr) => {
        define_rdrr_struct!($name);

        impl Instruction for $name
        {
            fn mnemonic(&self) -> String { $mnemonic.to_owned() }

            fn operands(&self) -> Vec<OperandInfo> {
                vec![
                    OperandInfo::input(self.lhs.clone()),
                    OperandInfo::input(self.rhs.clone()),
                ]
            }

            fn operands_mut(&mut self) -> Vec<&mut Operand> {
                vec![&mut self.lhs, &mut self.rhs]
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

define_rdrr!(ADDRdRr,  "add");
define_rdrr!(ADCRdRr,  "adc");
define_rdrr!(SUBRdRr,  "sub");
define_rdrr!(SBCRdRr,  "sbc");
define_rdrr!(MULRdRr,  "mul"); // FIXME: this actually writes result to r0
define_rdrr!(ANDRdRr,  "and");
define_rdrr!(ORRdRr,   "or");
define_rdrr!(EORRdRr,  "eor");
define_pure_rdrr!(CPSERdRr, "cpse");
define_pure_rdrr!(CPRdRr,   "cp");
define_pure_rdrr!(CPCRdRr,  "cpc");

