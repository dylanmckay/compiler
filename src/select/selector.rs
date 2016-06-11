use mir;

/// Selects instructions.
pub struct Selector<Out>
{
    f: Box<FnMut(&mir::Node) -> Option<Out>>,
}

impl<Out> Selector<Out>
{
    /// Creates a new instruction selector.
    pub fn new<F>(f: F) -> Self
        where F: Into<Box<FnMut(&mir::Node) -> Option<Out>>> {
        Selector {
            f: f.into(),
        }
    }

    pub fn select(&mut self, dag: mir::Dag) -> Vec<Out> {
        let mut results = Vec::new();

        for register in dag.registers {
            let outputs = self.select_node(register.value);
            results.extend(outputs);
        }

        results
    }

    pub fn select_node(&mut self, node: mir::Node) -> Vec<Out> {
        // check if the node can be directly selected.
        let results = if let Some(result) = (self.f)(&node) {
            vec![result]
        } else {
            // The node must be decomposed into more nodes.
            unimplemented!();
        };

        unimplemented!();
    }
}

