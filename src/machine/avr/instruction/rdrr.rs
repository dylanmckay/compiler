use {Instruction, Operand, OperandInfo, EncodedInstruction, SideEffects};
use avr::registers::GPR8;
use mir;
use std;

macro_rules! define_rdrr {
    ($name:ident, $mnemonic:expr) => {
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

                let rd = Operand::VirtualRegister { id: dest_reg.register_id, class: &GPR8 };
                let rr = Operand::VirtualRegister { id: source_reg.register_id, class: &GPR8 };

                Box::new(Self::new(rd, rr))
            }
        }

        impl Instruction for $name
        {
            fn mnemonic(&self) -> String { $mnemonic.to_owned() }
            fn operands(&self) -> Vec<OperandInfo> {
                vec![
                    OperandInfo { value: self.lhs.clone() },
                    OperandInfo { value: self.rhs.clone() },
                ]
            }

            fn side_effects(&self) -> SideEffects {
                SideEffects::none()
            }

            fn encode(&self) -> EncodedInstruction {
                unimplemented!();
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                try!(write!(fmt, "{} ", self.mnemonic()));

                let operands: Vec<_> = self.operands().iter().map(|op| format!("{:?}", op)).collect();
                try!(write!(fmt, "{}", operands.join(", ")));

                Ok(())
            }
        }
    }
}

define_rdrr!(ADDRdRr,  "add");
define_rdrr!(ADCRdRr,  "adc");
define_rdrr!(SUBRdRr,  "sub");
define_rdrr!(SBCRdRr,  "sbc");
define_rdrr!(MULRdRr,  "mul");
define_rdrr!(ANDRdRr,  "and");
define_rdrr!(ORRdRr,   "or");
define_rdrr!(EORRdRr,  "eor");
define_rdrr!(CPSERdRr, "cpse");
define_rdrr!(CPRdRr,   "cp");
define_rdrr!(CPCRdRr,  "cpc");

