use Node;
use ir;

use build;
use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Register {
    pub id: util::Id,
    pub value: Node,
}

impl Register {
    pub fn new(value: Node) -> Self {
        Register {
            id: util::Id::next(),
            value: value,
        }
    }

    pub fn map<F>(self, mut f: F) -> Self
        where F: FnMut(Node) -> Node {
        Register {
            id: self.id,
            value: f(self.value),
        }
    }
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Dag
{
    pub registers: Vec<Register>,
}

impl Dag
{
    pub fn new<I>(nodes: I) -> Self
        where I: IntoIterator<Item=Node> {
        Dag {
            registers: nodes.into_iter().map(Register::new).collect(),
        }
    }

    pub fn from_block(block: &ir::Block) -> Dag {
        build::from_block(block)
    }
}

