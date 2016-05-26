use {Value,Node,Dag,OpCode};
use ir;

pub fn from_block(block: &ir::Block) -> Dag<Value> {
    let nodes: Vec<Node<_>> = block.values().map(|value| {

        if let ir::Expression::Instruction(ref i) = value.node {
            self::value_from_instruction(i)
        } else {
            panic!("all block-level values should be instructions: \
                   expected instruction but got: {:?}", value.node);
        }
    }).collect();

    Dag::new(nodes)
}

pub fn value_from_instruction(inst: &ir::Instruction) -> Node<Value> {
    use ir::Instruction;
    use ir::Binary;

    match *inst {
        Instruction::Add(ref i) => {
            let (lhs, rhs) = i.operands();

            Node::new(
                OpCode::Add,
                (vec![Value::from_ir(lhs), Value::from_ir(rhs)].into_iter()),
            )
        },
        Instruction::Sub(ref i) => {
            let (lhs, rhs) = i.operands();

            Node::new(
                OpCode::Sub,
                vec![Value::from_ir(lhs), Value::from_ir(rhs)].into_iter(),
            )
        },
        Instruction::Mul(ref i) => {
            let (lhs, rhs) = i.operands();

            Node::new(
                OpCode::Mul,
                vec![Value::from_ir(lhs), Value::from_ir(rhs)].into_iter(),
            )
        },
        Instruction::Div(ref i) => {
            let (lhs, rhs) = i.operands();

            Node::new(
                OpCode::Div,
                vec![Value::from_ir(lhs), Value::from_ir(rhs)].into_iter(),
            )
        },
        Instruction::Shl(ref i) => {
            let (lhs, rhs) = i.operands();

            Node::new(
                OpCode::Shl,
                vec![Value::from_ir(lhs), Value::from_ir(rhs)].into_iter(),
            )
        },
        Instruction::Shr(ref i) => {
            let (lhs, rhs) = i.operands();

            Node::new(
                OpCode::Shr,
                vec![Value::from_ir(lhs), Value::from_ir(rhs)].into_iter(),
            )
        },
        Instruction::Return(ref i) => {
            match i.subvalue() {
                Some(value) => Node::new(OpCode::Ret, vec![Value::from_ir(value)].into_iter()),
                None => Node::new(OpCode::Ret, vec![])
            }
        },
        _ => unimplemented!(),
    }
}

