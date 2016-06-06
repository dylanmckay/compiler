use {Node,Dag,OpCode};
use ir;

pub fn from_block(block: &ir::Block) -> Dag {
    let nodes: Vec<Node> = block.values().map(|value| {

        if let ir::Expression::Instruction(ref i) = value.node {
            self::value_from_instruction(i)
        } else {
            panic!("all block-level values should be instructions: \
                   expected instruction but got: {:?}", value.node);
        }
    }).collect();

    Dag::new(nodes)
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

