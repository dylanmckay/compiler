use OpCode;
use ir;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Node
{
    opcode: OpCode,
    operands: Vec<Value>,
}

macro_rules! define_ctor {
    ($name:ident, $opcode:expr) => {
        pub fn $name<I>(operands: I) -> Self
            where I: IntoIterator<Item=Value> {
            Node::new($opcode, operands)
        }
    }
}

impl Node
{
    pub fn new<I>(opcode: OpCode,
                  operands: I) -> Self
        where I: IntoIterator<Item=Value> {
        Node {
            opcode: opcode,
            operands: operands.into_iter().collect(),
        }
    }

    pub fn from_instruction(inst: &ir::Instruction) -> Self {
        use ir::Instruction;
        use ir::Binary;

        match *inst {
            Instruction::Add(ref i) => {
                let (lhs, rhs) = i.operands();
                Self::add(vec![Value::from_ir(lhs), Value::from_ir(rhs)].into_iter())
            },
            Instruction::Sub(ref i) => {
                let (lhs, rhs) = i.operands();
                Self::sub(vec![Value::from_ir(lhs), Value::from_ir(rhs)].into_iter())
            },
            Instruction::Mul(ref i) => {
                let (lhs, rhs) = i.operands();
                Self::mul(vec![Value::from_ir(lhs), Value::from_ir(rhs)].into_iter())
            },
            Instruction::Div(ref i) => {
                let (lhs, rhs) = i.operands();
                Self::div(vec![Value::from_ir(lhs), Value::from_ir(rhs)].into_iter())
            },
            Instruction::Shl(ref i) => {
                let (lhs, rhs) = i.operands();
                Self::shl(vec![Value::from_ir(lhs), Value::from_ir(rhs)].into_iter())
            },
            Instruction::Shr(ref i) => {
                let (lhs, rhs) = i.operands();
                Self::shr(vec![Value::from_ir(lhs), Value::from_ir(rhs)].into_iter())
            }
            _ => unimplemented!(),
        }
    }

    define_ctor!(add, OpCode::Add);
    define_ctor!(sub, OpCode::Sub);
    define_ctor!(mul, OpCode::Mul);
    define_ctor!(div, OpCode::Div);
    define_ctor!(shl, OpCode::Shl);
    define_ctor!(shr, OpCode::Shr);
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Value
{
    Integer {
        bit_width: u16,
        value: i64,
    },
}

impl Value
{
    pub fn from_ir(value: &ir::Value) -> Self {
        use num::traits::ToPrimitive;

        match value.expression {
            ir::Expression::Literal(ref literal) => {
                match *literal {
                    ir::value::Literal::Integer(ref i) => {
                        Value::Integer {
                            bit_width: i.integer_ty().width(),
                            value: i.value().to_i64().unwrap(),
                        }
                    },
                    _ => unimplemented!(),
                }
            },
            _ => unimplemented!(),
        }
    }
}
