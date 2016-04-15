use Node;
use Value;
use ir;
use build;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Dag<V>
{
    nodes: Vec<Node<V>>,
}

impl<V> Dag<V>
{
    pub fn new<I>(nodes: I) -> Self
        where I: IntoIterator<Item=Node<V>> {
        Dag {
            nodes: nodes.into_iter().collect(),
        }
    }
}

impl Dag<Value>
{
    pub fn from_block(block: &ir::Block) -> Dag<Value> {
        build::from_block(block)
    }
}

