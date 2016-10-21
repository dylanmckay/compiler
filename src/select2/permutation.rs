use {Selectable, Pattern, PatternValue};
use mir;

/// A permutation of a node.
pub struct Node<S: Selectable+'static, V: PatternValue>
{
    pub kind: NodeKind<S, V>,
}

/// Either a matched node, or a list of other permutations.
pub enum NodeKind<S: Selectable+'static, V: PatternValue>
{
    /// A single (possibly matched) node.
    Leaf {
        node: mir::Node,
        pattern: Option<Pattern<S,V>>,
    },
    /// A list of other nodes.
    Branch {
        nodes: Vec<Node<S,V>>,
    },
}

impl<S,V> Node<S,V>
    where S: Selectable+'static, V: PatternValue {
    /// Create a new unmatched permutation.
    pub fn unmatched(node: mir::Node) -> Self {
        Node {
            kind: NodeKind::Leaf {
                node: node,
                pattern: None,
            },
        }
    }
}

