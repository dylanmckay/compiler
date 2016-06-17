use {Instruction, OperandInfo, Operand, EncodedInstruction, SideEffects};
use mir;
use std;

macro_rules! define_simple {
    ($name:ident, $mnemonic:expr, $bits:expr) => {
        #[derive(Clone)]
        pub struct $name;

        impl $name
        {
            pub fn from_pattern(_: &mir::Node) -> Box<Instruction> {
                Box::new($name)
            }
        }

        impl Instruction for $name
        {
            fn mnemonic(&self) -> String { $mnemonic.to_owned() }

            fn operands(&self) -> Vec<OperandInfo> { vec![] }

            fn operands_mut(&mut self) -> Vec<&mut Operand> { vec![] }

            fn side_effects(&self) -> SideEffects {
                SideEffects::none()
            }

            fn encode(&self) -> EncodedInstruction {
                // FIXME: this is only for the 'RET' instruction.
                EncodedInstruction::from($bits)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(fmt, "{} ", self.mnemonic())
            }
        }
    }
}

define_simple!(RET,  "ret",  0b1001_0101_0000_1000u16);
define_simple!(RETI, "reti", 0b1001_0101_0001_1000u16);

