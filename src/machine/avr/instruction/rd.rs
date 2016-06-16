use {Instruction, Operand, OperandInfo, EncodedInstruction, SideEffects};
use mir;
use std;

macro_rules! define_rd_struct {
    ($name:ident) => {
        #[derive(Clone)]
        pub struct $name
        {
            pub rd: Operand,
        }

        impl $name
        {
            pub fn new(rd: Operand) -> Self {
                $name { rd: rd }
            }

            pub fn from_pattern(_node: &mir::Node) -> Box<Instruction> {
                unreachable!(); // there are no patterns
            }
        }
    }
}

macro_rules! define_rd {
    ($name:ident, $mnemonic:expr) => {
        define_rd_struct!($name);

        impl Instruction for $name
        {
            fn mnemonic(&self) -> String { $mnemonic.to_owned() }
            fn operands(&self) -> Vec<OperandInfo> {
                vec![
                    OperandInfo::input_output(self.rd.clone()),
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

define_rd!(INCRd,   "inc");
define_rd!(DECRd,   "dec");
define_rd!(PUSHRd,  "push");
define_rd!(POPRd,   "pop");
define_rd!(COMRd,   "com");
define_rd!(NEGRd,   "neg");
define_rd!(LSLRd,   "lsl");
define_rd!(LSRRd,   "lsr");
define_rd!(ASRRd,   "asr");
define_rd!(ROLRd,   "rol");
define_rd!(RORRd,   "ror");
define_rd!(SWAPRd,  "swap");

