use {Instruction, Operand, EncodedInstruction};
use mir;
use std;

macro_rules! define_rdi {
    ($name:ident, $mnemonic:expr) => {
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
                let _branch = node.expect_branch();
                let rd = Operand::Register(0);
                let i = Operand::Register(0);
                Box::new(Self::new(rd, i))
            }
        }

        impl Instruction for $name
        {
            fn mnemonic(&self) -> String { $mnemonic.to_owned() }
            fn operands(&self) -> Vec<Operand> {
                vec![self.rd.clone(), self.i.clone()]
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

define_rdi!(ADIWRdK,  "adiw");
define_rdi!(SUBIRdK,  "subi");
define_rdi!(SBCIRdK,  "sbci");
define_rdi!(ANDIRdK,  "andi");
define_rdi!(ORIRdK,   "ori");
define_rdi!(CPIRdK,   "cpi");
define_rdi!(LDIRdK,   "ldi");

