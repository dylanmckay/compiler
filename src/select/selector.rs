use mir;

/// Selects instructions.
pub struct Selector<Out>
{
    f: Box<FnMut(mir::Node) -> Option<Out>>,
}

impl<Out> Selector<Out>
{
    /// Creates a new instruction selector.
    pub fn new<F>(f: F) -> Self
        where F: Into<Box<FnMut(mir::Node) -> Option<Out>>> {
        Selector {
            f: f.into(),
        }
    }

    pub fn select(dag: mir::Dag) -> Vec<Out> {
        unimplemented!();
    }

    pub fn select_node(node: mir::Node) -> Vec<Out> {
        unimplemented!();
    }
}

