use mir;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct PatternNode
{
    pub opcode: mir::OpCode,
    pub operands: Vec<PatternOperand>,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum PatternOperand
{
    Immediate { width: u32 },
    Node(Box<PatternNode>),
}

#[derive(Clone, Debug)]
pub struct Pattern
{
    pub root: PatternNode,
}

