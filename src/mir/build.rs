use {Node,Dag,OpCode,Register};
use ir;
use util::{self, Identifiable};

use std::collections::HashMap;

pub fn from_function(func: &ir::Function) -> Vec<Dag> {
    let mut register_map: HashMap<util::Id, util::Id> = HashMap::new();

    func.blocks().map(|block| {
        let registers: Vec<Register> = block.values().map(|value| {

            match value.node {
                ir::Expression::Instruction(ref i) => {
                    Register::new(self::value_from_instruction(i))
                },
                ir::Expression::Register(ref r) => {
                    let old_id = r.get_id();
                    let value = self::value_from_instruction(r.value.node.expect_instruction());
                    let new_register = Register::new(value);

                    register_map.insert(old_id, new_register.id);

                    new_register
                },
                _ => {
                    panic!("all block-level values should be instructions: \
                           expected instruction but got: {:?}", value.node);
                },
            }
        }).collect();

        Dag::new(registers)
    }).collect()
}

pub fn value_from_instruction(inst: &ir::Instruction) -> Node {
    use ir::Instruction;
    use ir::Binary;

    match *inst {
        Instruction::Add(ref i) => {
            let (lhs, rhs) = i.operands();

            Node::branch(
                OpCode::Add,
                (vec![Node::from_ir(lhs), Node::from_ir(rhs)].into_iter()),
            )
        },
        Instruction::Sub(ref i) => {
            let (lhs, rhs) = i.operands();

            Node::branch(
                OpCode::Sub,
                vec![Node::from_ir(lhs), Node::from_ir(rhs)].into_iter(),
            )
        },
        Instruction::Mul(ref i) => {
            let (lhs, rhs) = i.operands();

            Node::branch(
                OpCode::Mul,
                vec![Node::from_ir(lhs), Node::from_ir(rhs)].into_iter(),
            )
        },
        Instruction::Div(ref i) => {
            let (lhs, rhs) = i.operands();

            Node::branch(
                OpCode::Div,
                vec![Node::from_ir(lhs), Node::from_ir(rhs)].into_iter(),
            )
        },
        Instruction::Shl(ref i) => {
            let (lhs, rhs) = i.operands();

            Node::branch(
                OpCode::Shl,
                vec![Node::from_ir(lhs), Node::from_ir(rhs)].into_iter(),
            )
        },
        Instruction::Shr(ref i) => {
            let (lhs, rhs) = i.operands();

            Node::branch(
                OpCode::Shr,
                vec![Node::from_ir(lhs), Node::from_ir(rhs)].into_iter(),
            )
        },
        Instruction::Return(ref i) => {
            match i.subvalue() {
                Some(value) => Node::branch(OpCode::Ret, vec![Node::from_ir(value)].into_iter()),
                None => Node::branch(OpCode::Ret, vec![])
            }
        },
        _ => unimplemented!(),
    }
}

