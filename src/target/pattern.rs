use mir;
use machine;
use select;
use std;

pub type Pattern = select::Pattern<PatternOperand>;
pub type PatternNode = select::PatternNode<PatternOperand>;
pub type MatchResult = select::MatchResult<PatternOperand>;

/// A pattern operand.
#[derive(Clone,PartialEq,Eq)]
pub enum PatternOperand
{
    Immediate { width: u32 },
    Register(&'static machine::RegisterClass),
}

pub type Adjustment = ();

impl select::PatternValue for PatternOperand {
    type Adjustment = Adjustment;

    fn matches(&self, value: &mir::Value) -> MatchResult {
        match *self {
            PatternOperand::Immediate { width } => {
                match *value {
                    mir::Value::ConstantInteger { bit_width, .. } => {
                        if bit_width <= width { select::MatchResult::Perfect } else { select::MatchResult::None }
                    },
                    _ => select::MatchResult::None,
                }
            },
            PatternOperand::Register(class) => {
                if value.ty().bit_width() <= class.bit_width {
                    select::MatchResult::Perfect
                } else {
                    select::MatchResult::None
                }
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

