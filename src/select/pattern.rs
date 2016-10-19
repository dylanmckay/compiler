use Adjustment;
use Selectable;

use mir;
use std;
use util;

/// A pattern.
pub struct Pattern<S: Selectable + 'static, V: PatternValue>
{
    pub root: PatternNode<V>,
    pub factory: fn(&mir::Node) -> S,
}

impl<S: Selectable + 'static, V: PatternValue> Clone for Pattern<S, V>
{
    fn clone(&self) -> Self {
        Pattern {
            root: self.root.clone(),
            factory: self.factory,
        }
    }
}

/// A node in the pattern tree.
#[derive(Clone,PartialEq,Eq)]
pub struct PatternNode<V: PatternValue>
{
    pub id: util::Id,
    pub opcode: mir::OpCode,
    pub operands: Vec<PatternOperand<V>>,
}

#[derive(Clone,PartialEq,Eq)]
pub enum PatternOperand<V: PatternValue>
{
    /// A node.
    Node(Box<PatternNode<V>>),
    /// A value.
    Value {
        /// The name of the value.
        name: String,
        /// The underlying value.
        value: V,
    },
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum MatchResult<V: PatternValue>
{
    Perfect,
    Partial(Vec<Adjustment<V>>),
    None,
}

impl<V: PatternValue> MatchResult<V>
{
    pub fn adjust(adjustment: Adjustment<V>) -> Self {
        MatchResult::Partial(vec![adjustment])
    }

    pub fn is_perfect(&self) -> bool {
        if let MatchResult::Perfect = *self { true } else { false }
    }

    pub fn is_similar(&self) -> bool {
        match *self {
            MatchResult::Perfect |
                MatchResult::Partial(..) => true,
            MatchResult::None => false,
        }
    }
}

impl<V: PatternValue> std::ops::Add for MatchResult<V>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        use MatchResult::*;
        match (self, rhs) {
            // If something can't match, the entire pattern can't.
            (None, _) |
            (_, None) => MatchResult::None,
            (Partial(mut a1), Partial(a2)) => {
                a1.extend(a2.into_iter());
                Partial(a1)
            },
            (Perfect, Partial(a)) |
            (Partial(a), Perfect) => Partial(a),
            (Perfect, Perfect) => MatchResult::Perfect,
        }
    }
}

impl<S: Selectable, V: PatternValue> Pattern<S, V>
{
    pub fn matches(&self, node: &mir::Node) -> MatchResult<V> {
        if let mir::Node::Branch(ref branch) = *node {
            self.root.matches(branch)
        } else {
            MatchResult::None
        }
    }
}

impl<V: PatternValue> PatternNode<V>
{
    pub fn matches(&self, branch: &mir::Branch) -> MatchResult<V> {
        if self.opcode == branch.opcode && self.operands.len() == branch.operands.len() {
            self.operands.iter().zip(branch.operands.iter()).
                fold(MatchResult::Perfect, |result, (pat_op, mir_op)| result + pat_op.matches(mir_op))
        } else {
            MatchResult::None
        }
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
    pub fn matches(&self, node: &mir::Node) -> MatchResult<V> {
        match *self {
            PatternOperand::Value { ref name, ref value } => {
                if let mir::Node::Leaf(ref mir_val) = *node {
                    value.matches(mir_val)
                } else {
                    MatchResult::adjust(Adjustment::demote_to_register(node))
                }
            },
            PatternOperand::Node(ref pat_node) => {
                if let mir::Node::Branch(ref mir_branch) = *node {
                    pat_node.matches(mir_branch)
                } else {
                    MatchResult::None
                }
            },
        }
    }
}

/// A value.
pub trait PatternValue : Sized + Clone + std::fmt::Debug
{
    type Adjustment: Clone + PartialEq + Eq + std::fmt::Debug;

    fn matches(&self, value: &mir::Value) -> MatchResult<Self>;
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct DummyPatternValue;

impl PatternValue for DummyPatternValue {
    type Adjustment = ();
    fn matches(&self, _value: &mir::Value) -> MatchResult<Self> { unreachable!() }
}

impl<S: Selectable, V: PatternValue> std::fmt::Debug for Pattern<S, V>
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.root, fmt)
    }
}

impl<V: PatternValue> std::fmt::Debug for PatternNode<V>
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "({} {:?})", self.opcode.mnemonic(), self.operands)
    }
}

impl<V: PatternValue> std::fmt::Debug for PatternOperand<V>
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PatternOperand::Node(ref node) => write!(fmt, "({:?})", node),
            PatternOperand::Value { ref value, .. } => write!(fmt, "{:?}", value),
        }
    }
}

