use Legalizer;

use mir;

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Action
{
    /// The operation is already legal.
    Legal,
    /// The operation needs to be expanded into several
    /// simpler operations.
    Expand,
    /// The operands need to be promoted into
    /// bigger types.
    Promote,
}

impl Action
{
    pub fn perform_on(&self, node: mir::Node, legalizer: &Legalizer) -> mir::Node {
        match *self {
            Action::Legal => node,
            Action::Expand => self::expand(legalizer, node),
            Action::Promote => self::promote(legalizer, node),
        }
    }
}

pub fn expand(_context: &Legalizer, _node: mir::Node) -> mir::Node
{
    unimplemented!();
}

pub fn promote(_context: &Legalizer, node: mir::Node) -> mir::Node
{
    match node {
        mir::Node::Branch { opcode, operands } => {

        },
        mir::Node::Leaf => {

        },
    }
}

