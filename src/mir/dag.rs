use Node;
use ir;

use build;
use verifier;

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

    pub fn from_function(function: &ir::Function) -> Vec<Dag> {
        build::from_function(function)
    }

    pub fn validate(&self) -> verifier::Result {
        verifier::verify_dag(self)
    }
}

