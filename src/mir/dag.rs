use Node;
use Value;
use ir;
use build;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Dag
{
    pub nodes: Vec<Node>,
}

impl Dag
{
    pub fn new<I>(nodes: I) -> Self
        where I: IntoIterator<Item=Node> {
        Dag {
            nodes: nodes.into_iter().collect(),
        }
    }

    pub fn from_block(block: &ir::Block) -> Dag {
        build::from_block(block)
    }
}

