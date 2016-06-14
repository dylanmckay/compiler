use mir;
use machine;
use select;
use std;

pub type Pattern = select::Pattern<PatternOperand>;
pub type PatternNode = select::PatternNode<PatternOperand>;

/// A pattern operand.
#[derive(Clone,PartialEq,Eq)]
pub enum PatternOperand
{
    Immediate { width: u32 },
    Register(&'static machine::RegisterClass),
}

/// An adjustment to a pattern.
///
/// Not all possible patterns are an identical match to
/// the MIR. To accomodate this, we have the concept of a
/// pattern _adjustment_.
///
/// When matching patterns, we can have several adjustments
/// which define permutations that would need to be made to the
/// original pattern in order to match.
///
/// We can then look at all the adjustments and figure out which
/// pattern is the most optimal to select.
///
/// Take the case where we have
///
/// ```
/// (add %a, (add %foo, %bar))
/// ```
///
/// This will likely have to have an adjustment to demote the
/// nested addition to a register, so that the code becomes.
///
/// ```
/// %tmp = (add %foo, %bar)
/// (add %a, %tmp)
/// ```
#[derive(Clone,PartialEq,Eq)]
pub enum Adjustment
{
    /// Demotes a operand to a register.
    DemoteToRegister {
        operand_number: u32,
        class: &'static machine::RegisterClass,
    }
}

impl select::PatternValue for PatternOperand {
    type Adjustment = Adjustment;

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

