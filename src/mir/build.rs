use {Node,Value,Dag,OpCode,Register,Type};
use ir;
use util::{self, Identifiable};

use std::collections::HashMap;

/// A MIR building context.
/// Should only be used for a single function.
struct Context
{
    register_map: HashMap<util::Id, util::Id>,
    parameter_map: HashMap<util::Id, util::Id>,
}

impl Context
{
    fn new() -> Self {
        Context {
            register_map: HashMap::new(),
            parameter_map: HashMap::new(),
        }
    }

    fn put_register(&mut self, register: &ir::Register) -> Register {
        let old_id = register.get_id();
        let value = self::node_from_instruction(self, register.value.node.expect_instruction());
        let new_register = Register::new(value);

        self.register_map.insert(old_id, new_register.id);

        new_register
    }

    fn put_parameter_id(&mut self, id: util::Id) {
        // TODO: in the future, we need to store MIR arguments
        // here.
        let new_parameter_id = util::Id::next();
        self.parameter_map.insert(id, new_parameter_id);
    }

    fn map_parameter_id(&self, id: util::Id) -> util::Id {
        self.parameter_map.get(&id).expect("this parameter can not be found").clone()
    }
}

pub fn from_function(func: &ir::Function) -> Vec<Dag> {
    let mut context = Context::new();

    for param in func.signature.parameters() {
        context.put_parameter_id(param.get_id());
    }

    func.blocks().map(|block| {
        let registers: Vec<Register> = block.values().map(|value| {
            self::register_from_value(&mut context, value)
        }).collect();

        Dag::new(registers)
    }).collect()
}

fn register_from_value(context: &mut Context, value: &ir::Value) -> Register {
    match value.node {
        ir::Expression::Register(ref r) => context.put_register(r),
        _ => Register::new(self::node_from_value(context, value)),
    }
}

fn node_from_value(context: &mut Context, value: &ir::Value) -> Node {
    use num::traits::ToPrimitive;

    match value.node {
        ir::Expression::Instruction(ref i) => self::node_from_instruction(context, i),
        ir::Expression::Register(ref r) => panic!("registers can only be used at the top level"),
        ir::Expression::Literal(ref literal) => {
            match *literal {
                ir::value::Literal::Integer(ref i) => {
                    Node::leaf(Value::ConstantInteger {
                        bit_width: i.integer_ty().width() as _,
                        value: i.value().to_i64().unwrap(),
                    })
                },
                _ => unimplemented!(),
            }
        },
        ir::Expression::ArgumentRef(ref r) => {
            let id = context.map_parameter_id(r.param_id);
            let ty = self::convert_type(&r.ty).unwrap();

            Node::leaf(Value::ArgumentRef {
                id: id,
                ty: ty,
            })
        },
        _ => {
            panic!("do not know how to handle this IR value: {:#?}", value.node);
        },
    }
}

fn node_from_instruction(context: &mut Context, inst: &ir::Instruction) -> Node {
    use ir::Instruction;
    use ir::Binary;

    match *inst {
        Instruction::Add(ref i) => {
            let (lhs, rhs) = i.operands();

            Node::branch(
                OpCode::Add,
                (vec![node_from_value(context, lhs), node_from_value(context, rhs)].into_iter()),
            )
        },
        Instruction::Sub(ref i) => {
            let (lhs, rhs) = i.operands();

            Node::branch(
                OpCode::Sub,
                vec![node_from_value(context, lhs), node_from_value(context, rhs)].into_iter(),
            )
        },
        Instruction::Mul(ref i) => {
            let (lhs, rhs) = i.operands();

            Node::branch(
                OpCode::Mul,
                vec![node_from_value(context, lhs), node_from_value(context, rhs)].into_iter(),
            )
        },
        Instruction::Div(ref i) => {
            let (lhs, rhs) = i.operands();

            Node::branch(
                OpCode::Div,
                vec![node_from_value(context, lhs), node_from_value(context, rhs)].into_iter(),
            )
        },
        Instruction::Shl(ref i) => {
            let (lhs, rhs) = i.operands();

            Node::branch(
                OpCode::Shl,
                vec![node_from_value(context, lhs), node_from_value(context, rhs)].into_iter(),
            )
        },
        Instruction::Shr(ref i) => {
            let (lhs, rhs) = i.operands();

            Node::branch(
                OpCode::Shr,
                vec![node_from_value(context, lhs), node_from_value(context, rhs)].into_iter(),
            )
        },
        Instruction::Return(ref i) => {
            match i.subvalue() {
                Some(value) => Node::branch(OpCode::Ret, vec![node_from_value(context, value)].into_iter()),
                None => Node::branch(OpCode::Ret, vec![])
            }
        },
        _ => unimplemented!(),
    }
}

/// Converts an IR type to a MIR type.
/// Returns `Some` if the type could be converted.
fn convert_type(ty: &ir::Type) -> Option<Type>
{
    match *ty {
        ir::Type::Integer(ref i) => Some(Type::Integer { bit_width: i.bit_width as _ }),
        _ => unimplemented!(),
    }
}

