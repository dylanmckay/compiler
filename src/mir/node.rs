use OpCode;
use Value;
use Type;

use ir;

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Node
{
    Branch {
        opcode: OpCode,
        operands: Vec<Node>,
    },
    Leaf {
        value: Value,
    },
}

impl Node
{
    pub fn branch<I>(opcode: OpCode,
                     operands: I) -> Self
        where I: IntoIterator<Item=Self> {
        Node::Branch {
            opcode: opcode,
            operands: operands.into_iter().collect(),
        }
    }

    pub fn leaf(value: Value) -> Self {
        Node::Leaf {
            value: value,
        }
    }

    pub fn from_ir(value: &ir::Value) -> Self {
        use num::traits::ToPrimitive;

        match value.node {
            ir::Expression::Literal(ref literal) => {
                match *literal {
                    ir::value::Literal::Integer(ref i) => {
                        Node::leaf(Value::Integer {
                            bit_width: i.integer_ty().width(),
                            value: i.value().to_i64().unwrap(),
                        })
                    },
                    _ => unimplemented!(),
                }
            },
            _ => unimplemented!(),
        }
    }

    pub fn result_types(&self) -> ::std::vec::IntoIter<Type> {
        match *self {
            Node::Branch { opcode, ref operands } => match opcode {
                OpCode::Add |
                OpCode::Sub |
                OpCode::Mul |
                OpCode::Div |
                OpCode::Shl |
                OpCode::Shr => {
                    unimplemented!();
                },
                OpCode::Ret => {
                    vec![Type::Nothing].into_iter()
                }
            },
            Node::Leaf { ref value } => vec![value.ty()].into_iter(),
        }
    }
}

