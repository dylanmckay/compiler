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

    pub fn i(width: u32, value: i64) -> Self {
        Self::leaf(Value::i(width, value))
    }

    pub fn sext(bit_width: u32, value: Self) -> Self {
        Self::branch(OpCode::Sext, vec![Self::i(32, bit_width as _), value])
    }

    pub fn zext(bit_width: u32, value: Self) -> Self {
        Self::branch(OpCode::Zext, vec![Self::i(32, bit_width as _), value])
    }

    pub fn from_ir(value: &ir::Value) -> Self {
        use num::traits::ToPrimitive;

        match value.node {
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
                    vec![Type::Nothing]
                }
                OpCode::Sext |
                OpCode::Zext => {
                    assert_eq!(operands.len(), 2);
                    let bit_width = operands.first().unwrap().expect_leaf().
                        expect_constant_integer();

                    vec![Type::Integer { bit_width: bit_width as _ }]
                },
            },
            Node::Leaf { ref value } => vec![value.ty()]
        }.into_iter()
    }

    // FIXME: get rid of this so we can support multiple
    // results
    pub fn ty(&self) -> Type {
        let mut result_types = self.result_types();
        let first_result = result_types.next().unwrap();
        first_result
    }

    pub fn expect_leaf(&self) -> &Value {
        if let Node::Leaf { ref value } = *self {
            value
        } else {
            panic!("expected a leaf but got {:?}", self);
        }
    }
}

