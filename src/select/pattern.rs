use mir;
use std;

/// A pattern.
#[derive(Clone)]
pub struct Pattern<V>
{
    pub root: PatternNode<V>,
}

/// A node in the pattern tree.
#[derive(Clone,PartialEq,Eq)]
pub struct PatternNode<V>
{
    pub opcode: mir::OpCode,
    pub operands: Vec<V>,
}

#[derive(Clone,PartialEq,Eq)]
pub enum PatternOperand<V>
{
    /// A node.
    Node(Box<PatternNode<V>>),
    /// A value.
    Value(V),
}

/// A value.
pub trait PatternValue : std::fmt::Debug { }

impl<V: PatternValue> std::fmt::Debug for Pattern<V>
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.root, fmt)
    }
}

impl<V: PatternValue> std::fmt::Debug for PatternNode<V>
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "({:?} {:?})", self.opcode, self.operands)
    }
}

impl<V: PatternValue> std::fmt::Debug for PatternOperand<V>
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PatternOperand::Node(ref node) => write!(fmt, "({:?})", node),
            PatternOperand::Value(ref val) => write!(fmt, "{:?}", val),
        }
    }
}

