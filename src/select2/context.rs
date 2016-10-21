use {Node, Selectable, PatternValue};
use mir;

/// An instruction selection context for a function.
pub struct Context<S: Selectable + 'static, V: PatternValue>
{
    pub permutations: Vec<Node<S,V>>,
}

impl<S,V> Context<S,V>
    where S: Selectable + 'static, V: PatternValue
{
    pub fn new(nodes: Vec<mir::Node>) -> Self {
        Context {
            permutations: nodes.into_iter().map(Node::unmatched).collect(),
        }
    }

    pub fn select(self) -> Vec<S> {
        unimplemented!();
    }
}

