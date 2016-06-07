use {Node, Register, Value};

/// Given a node, splits one of the operands out into a register
/// and replaces the operand into a reference to that register.
pub fn split_node(parent: &mut Node, operand_num: u32) -> Register {
    match parent {
        // We can only split branches
        &mut Node::Branch { ref mut operands, .. } => {
            let old_operand = operands[operand_num as usize].clone();

            // At this point, if the old operand had several return
            // values, they should already be in separate registers.
            assert_eq!(old_operand.result_types().len(), 1);

            // Create a register and move the operand into it.
            let new_register = Register::new(old_operand);

            // Replace the old operand with a reference to the register.
            operands[operand_num as usize] = Node::leaf(Value::reference_register(
                &new_register,
                0, // Result number
            ));

            new_register
        },
        _ => panic!("only branches can be split"),
    }
}

#[cfg(test)]
mod test
{
    use super::*;
    use {Node, Value, OpCode};

    #[test]
    fn test_split_node() {
        let child = Node::sext(16, Node::i(8, 50));
        let mut parent = Node::add(&[Node::i(16, 5), child.clone()]);

        let new_register = split_node(&mut parent, 1);
        assert_eq!(new_register.value, child);

        if let Node::Branch { opcode, operands } = parent {
            assert_eq!(opcode, OpCode::Add);

            assert_eq!(
                operands[1],
                Node::leaf(Value::reference_register(&new_register, 0))
            );
        } else {
            unreachable!();
        };
    }
}

