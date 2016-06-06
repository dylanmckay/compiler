use {Operation, Action};
use legalize;

use mir;

/// A selection context.
pub struct Context
{
    operations: Vec<Operation>,
    /// The width of a byte on the target.
    pub byte_width: u32,
}

impl Context
{
    pub fn new(byte_width: u32) -> Self {
        Context {
            operations: Vec::new(),
            byte_width: byte_width,
        }
    }

    pub fn on(&mut self, opcode: mir::OpCode, types: &[mir::Type], action: Action) {
        self.operations.push(Operation {
            opcode: opcode,
            result_types: types.to_owned(),
            action: action,
        });
    }

    pub fn legalize(&self, dag: mir::Dag) -> mir::Dag {
        mir::Dag {
            nodes: dag.nodes.into_iter().map(|node| self.legalize_node(node)).collect(),
        }
    }

    fn legalize_node(&self, node: mir::Node) -> mir::Node {
        match node {
            mir::Node::Branch { opcode, operands } => {
                let operands = operands.into_iter().map(|operand| self.legalize_node(operand)).collect();

                let node = mir::Node::Branch {
                    opcode: opcode,
                    operands: operands,
                };

                match self.legalization_action(&node) {
                    Action::Legal => node,
                    Action::Expand => legalize::expand(self, node),
                    Action::Promote => legalize::promote(self, node),
                }
            },
            mir::Node::Leaf { value } => {
                mir::Node::Leaf { value: value }
            },
        }
    }

    fn legalization_action(&self, node: &mir::Node) -> Action {
        match *node {
            mir::Node::Branch { opcode, .. } => {
                let predefined_action = self.operations.iter().find(|op| {
                    op.opcode == opcode &&
                        op.result_types.iter().cloned().eq(node.result_types())
                });

                if let Some(operation) = predefined_action {
                    operation.action
                } else {
                    unimplemented!(); // no action for this operation
                }
            },
            mir::Node::Leaf { .. } => {
                // FIXME: not all values are legal
                Action::Legal
            },
        }
    }
}


