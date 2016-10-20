use mir;

/// A permutation of a node.
pub struct Node
{
    pub kind: NodeKind,
}

/// Either a matched node, or a list of other permutations.
pub enum NodeKind
{
    /// A single (possibly matched) node.
    Leaf {
        node: mir::Node,
    },
    /// A list of other nodes.
    Branch {
        nodes: Vec<Node>,
    },
}

