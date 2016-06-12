use mir;
use machine;
use std;

#[derive(Clone)]
pub struct Pattern
{
    pub root: PatternNode,
}

#[derive(Clone,PartialEq,Eq)]
pub struct PatternNode
{
    pub opcode: mir::OpCode,
    pub operands: Vec<PatternOperand>,
}

#[derive(Clone,PartialEq,Eq)]
pub enum PatternOperand
{
    Immediate { width: u32 },
    Register(&'static machine::RegisterClass),
    Node(Box<PatternNode>),
}

impl std::fmt::Debug for Pattern
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.root, fmt)
    }
}

impl std::fmt::Debug for PatternNode
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "({:?} {:?})", self.opcode, self.operands)
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
            PatternOperand::Node(ref node) => {
                unimplemented!();
            },
        }
    }
}
