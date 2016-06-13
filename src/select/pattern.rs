use mir;
use std;

/// A pattern.
#[derive(Clone)]
pub struct Pattern<V: PatternValue>
{
    pub root: PatternNode<V>,
}

/// A node in the pattern tree.
#[derive(Clone,PartialEq,Eq)]
pub struct PatternNode<V: PatternValue>
{
    pub opcode: mir::OpCode,
    pub operands: Vec<PatternOperand<V>>,
}

#[derive(Clone,PartialEq,Eq)]
pub enum PatternOperand<V: PatternValue>
{
    /// A node.
    Node(Box<PatternNode<V>>),
    /// A value.
    Value(V),
}

impl<V: PatternValue> Pattern<V>
{
    pub fn matches(&self, node: &mir::Node) -> bool {
        if let mir::Node::Branch(ref branch) = *node {
            self.root.matches(branch)
        } else {
            false
        }
    }
}

impl<V: PatternValue> PatternNode<V>
{
    pub fn matches(&self, branch: &mir::Branch) -> bool {
        self.opcode == branch.opcode &&
            self.operands.iter().zip(branch.operands.iter()).
                all(|(pat_op, mir_op)| pat_op.matches(mir_op))
    }

    /// Gets the total area coverted by the tree.
    /// Each node is considered one unit. This is essentially
    /// the total number of nodes and values in the tree.
    pub fn area(&self) -> u32 {
        let mut area = 0;
        self.area_internal(&mut area);
        area
    }

    fn area_internal(&self, area: &mut u32) {
        *area += 1;

        for operand in self.operands.iter() {
            if let PatternOperand::Node(ref child_node) = *operand {
                child_node.area_internal(area);
            } else {
                *area += 1;
            }
        }
    }
}

impl<V: PatternValue> PatternOperand<V>
{
    pub fn matches(&self, node: &mir::Node) -> bool {
        match *self {
            PatternOperand::Value(ref pat_val) => {
                if let mir::Node::Leaf(ref mir_val) = *node {
                    pat_val.matches(mir_val)
                } else {
                    false
                }
            },
            PatternOperand::Node(ref pat_node) => {
                if let mir::Node::Branch(ref mir_branch) = *node {
                    pat_node.matches(mir_branch)
                } else {
                    false
                }
            },
        }
    }
}

/// A value.
pub trait PatternValue : Sized + Clone + std::fmt::Debug
{
    fn matches(&self, value: &mir::Value) -> bool;
}

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

