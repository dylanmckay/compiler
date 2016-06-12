use Node;
use ir;

use build;
use verifier;
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
    pub fn new<I>(registers: I) -> Self
        where I: IntoIterator<Item=Register> {
        Dag {
            registers: registers.into_iter().collect(),
        }
    }

    pub fn from_function(function: &ir::Function) -> Vec<Dag> {
        build::from_function(function)
    }

    pub fn validate(&self) -> verifier::Result {
        verifier::verify_dag(self)
    }
}

