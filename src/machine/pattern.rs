use RegisterClass;
use Instruction;
use mir;
use select;

use std;

pub type Pattern = select::Pattern<Box<Instruction>, PatternOperand>;
pub type PatternNode = select::PatternNode<PatternOperand>;
pub type MatchResult = select::MatchResult<PatternOperand>;

/// A pattern operand.
#[derive(Clone,PartialEq,Eq)]
pub enum PatternOperand
{
    Immediate { width: u32 },
    Register(&'static RegisterClass),
}

pub type Adjustment = ();

impl select::PatternValue for PatternOperand {
    type Adjustment = Adjustment;

    fn matches(&self, value: &mir::Value) -> MatchResult {
        match *self {
            PatternOperand::Immediate { width } => {
                match *value {
                    mir::Value::ConstantInteger(ref c) => {
                        if c.bit_width <= width { select::MatchResult::Perfect } else { select::MatchResult::None }
                    },
                    _ => select::MatchResult::None,
                }
            },
            PatternOperand::Register(class) => {
                if value.ty().bit_width() <= class.bit_width {
                    // If the value is already stored in a register.
                    if self::is_value_stored_in_register(value) {
                        select::MatchResult::Perfect
                    } else {
                        // We have to demote it into a register.
                        select::MatchResult::adjust(select::Adjustment::demote_to_register(&mir::Node::Leaf(value.clone())))
                    }
                } else {
                    select::MatchResult::None
                }
            },
        }
    }
}

impl select::Selectable for Box<Instruction>
{

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

fn is_value_stored_in_register(value: &mir::Value) -> bool {
    match *value {
        mir::Value::RegisterRef(..) => true,
        mir::Value::ArgumentRef { .. } => true,
        mir::Value::ConstantInteger(..) => false,
    }
}

