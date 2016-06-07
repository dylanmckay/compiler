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

    pub fn split_node(parent: mir::Node, argument_num: u32)
        -> mir::Node {
        let (opcode, operands) = match parent {
            mir::Node::Branch { opcode, operands } => (opcode, operands),
            _ => panic!("only branches can be split"),
        };

        let operands = operands.into_iter().enumerate().map(|(i, operand)| {
            if i == argument_num as _ {
                let result_number = 0;

                // Replace the child node with a register.
                mir::Node::leaf(mir::Value::register_ref(
                    0, // node number
                    result_number, // value number
                    operand.result_types().nth(result_number as _).unwrap(),
                ))
            } else {
                operand
            }
        }).collect();

        mir::Node::Branch {
            opcode: opcode,
            operands: operands,
        }
    }
}

