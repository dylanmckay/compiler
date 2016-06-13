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
    Node(Box<PatternNode>),
}

impl select::PatternValue for PatternOperand { }

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
            PatternOperand::Node(ref _node) => {
                unimplemented!();
            },
        }
    }
}

