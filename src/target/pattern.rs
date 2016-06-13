use mir;
use machine;
use select;
use std;

pub type Pattern = select::Pattern<PatternOperand>;
pub type PatternNode = select::PatternNode<PatternOperand>;

#[derive(Clone,PartialEq,Eq)]
pub enum PatternOperand
{
    Immediate { width: u32 },
    Register(&'static machine::RegisterClass),
}

impl select::PatternValue for PatternOperand {
    fn matches(&self, value: &mir::Value) -> bool {
        match *self {
            PatternOperand::Immediate { width } => {
                match *value {
                    mir::Value::ConstantInteger { bit_width, .. } => bit_width <= width,
                    _ => false,
                }
            },
            PatternOperand::Register(class) => {
                value.ty().bit_width() <= class.bit_width
            },
        }
    }
}

impl std::fmt::Debug for PatternOperand
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PatternOperand::Immediate { width } => {
                write!(fmt, "i{}imm", width)
            },
            PatternOperand::Register(class) => {
                write!(fmt, "{}", class.name)
            },
        }
    }
}

