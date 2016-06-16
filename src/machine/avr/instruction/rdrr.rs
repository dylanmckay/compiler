use {Instruction, Operand, EncodedInstruction};
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
                let _branch = node.expect_branch();
                let rd = Operand::Register(0);
                let rr = Operand::Register(0);
                Box::new(Self::new(rd, rr))
            }
        }

        impl Instruction for $name
        {
            fn mnemonic(&self) -> String { $mnemonic.to_owned() }
            fn operands(&self) -> Vec<Operand> {
                vec![self.lhs.clone(), self.rhs.clone()]
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
define_rdrr!(MOVRdRr,  "mov");

