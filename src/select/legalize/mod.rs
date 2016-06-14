pub use self::action::Action;
pub use self::operation::Operation;

pub mod action;
pub mod operation;
pub mod default;

use mir;

/// A selection context.
pub struct Legalizer
{
    operations: Vec<Operation>,
    /// The width of a byte on the target.
    pub byte_width: u32,
}

impl Legalizer
{
    pub fn new(byte_width: u32) -> Self {
        Legalizer {
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
            nodes: dag.nodes.into_iter().map(|node| {
                self.legalize_node(node)
            }).collect()
        }
    }

    fn legalize_node(&self, node: mir::Node) -> mir::Node {
        match node {
            mir::Node::Branch(branch) => {
                let operands = branch.operands.into_iter().
                    map(|operand| self.legalize_node(operand)).collect();

                let node = mir::Node::Branch(mir::Branch {
                    opcode: branch.opcode,
                    operands: operands,
                });

                self.legalization_action(&node).perform_on(node, self)
            },
            mir::Node::Leaf(value) => {
                mir::Node::Leaf(value)
            },
        }
    }

    fn legalization_action(&self, node: &mir::Node) -> Action {
        match *node {
            mir::Node::Branch(ref branch) => {
                if branch.opcode == mir::OpCode::Set { return Action::Legal };

                let predefined_action = self.operations.iter().find(|op| {
                    op.opcode == branch.opcode &&
                        op.result_types.iter().cloned().eq(node.result_types())
                });

                if let Some(operation) = predefined_action {
                    operation.action
                } else {
                    panic!("no action for {:#?}", branch);
                }
            },
            mir::Node::Leaf(..) => {
                // FIXME: not all values are legal
                Action::Legal
            },
        }
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    pub mod legalization {
    }
}

