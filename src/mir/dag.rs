use Node;
use ir;

use build;
use verifier;
use expand;

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

    /// Expands this DAG from a flat structure into a tree structure
    /// where possible.
    ///
    /// Take a DAG like so:
    ///
    /// ```
    /// (set %a, (add i32 5, i32 10)
    /// (add %a, i32 15)
    /// ```
    ///
    /// This will turn this into a tree.
    ///
    /// ```
    /// (add (add i32 5, i32 10), i32 15)
    /// ```
    pub fn expand(self) -> Self {
        expand::dag(self)
    }

    pub fn filter_nodes<F>(mut self, f: F) -> Self
        where F: FnMut(&Node) -> bool {
        self.nodes = self.nodes.into_iter().filter(f).collect();
        self
    }

    pub fn validate(&self) -> verifier::Result {
        verifier::verify_dag(self)
    }
}

