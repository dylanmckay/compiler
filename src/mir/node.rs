use {OpCode, Value, Type, Register};

use utils;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Branch {
    pub opcode: OpCode,
    pub operands: Vec<Node>,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Node
{
    /// A branch node contains an opcode and several operands.
    Branch(Branch),
    /// A leaf node contains a plain value with no children.
    Leaf(Value),
}

impl Node
{
    /// Creates a new branch node.
    pub fn branch<I>(opcode: OpCode,
                     operands: I) -> Self
        where I: IntoIterator<Item=Self> {
        Node::Branch(Branch {
            opcode: opcode,
            operands: operands.into_iter().collect(),
        })
    }

    /// Creates a new leaf node.
    pub fn leaf(value: Value) -> Self {
        Node::Leaf(value)
    }

    /// Creates a new constant integer.
    pub fn i(width: u32, value: i64) -> Self {
        Self::leaf(Value::i(width, value))
    }

    /// Creates a sign extended value.
    pub fn sext(bit_width: u32, value: Self) -> Self {
        Self::branch(OpCode::Sext, vec![Self::i(32, bit_width as _), value])
    }

    /// Creates a zero extended value.
    pub fn zext(bit_width: u32, value: Self) -> Self {
        Self::branch(OpCode::Zext, vec![Self::i(32, bit_width as _), value])
    }

    /// Creates an sum of values node.
    pub fn add(addends: &[Self]) -> Self {
        Self::branch(OpCode::Add, addends.to_owned())
    }

    /// Creates a difference between values.
    pub fn sub(addends: &[Self]) -> Self {
        Self::branch(OpCode::Sub, addends.to_owned())
    }

    pub fn result_types(&self) -> ::std::vec::IntoIter<Type> {
        match *self {
            Node::Branch(ref branch) => match branch.opcode {
                OpCode::Add |
                OpCode::Sub |
                OpCode::Mul |
                OpCode::Div |
                OpCode::Shl |
                OpCode::Shr => {
                    // FIXME: check that all types are the same.
                    vec![branch.operands[0].ty()]
                },
                OpCode::Ret => {
                    vec![Type::Nothing]
                }
                OpCode::Sext |
                OpCode::Zext => {
                    assert_eq!(branch.operands.len(), 2);
                    let bit_width = branch.operands.first().unwrap().expect_leaf().
                        expect_constant_integer();

                    vec![Type::Integer { bit_width: bit_width as _ }]
                },
            },
            Node::Leaf(ref value) => vec![value.ty()]
        }.into_iter()
    }

    /// Splits an operand out of this branch into a new register.
    pub fn split_operand(&mut self, operand_number: u32) -> Register {
        utils::split_node(self, operand_number)
    }

    pub fn promote_to_register(&mut self) -> Register {
        let reg = Register::new(self.clone());
        *self = Self::leaf(Value::reference_register(&reg, 0));
        reg
    }

    // FIXME: get rid of this so we can support multiple
    // results
    pub fn ty(&self) -> Type {
        let mut result_types = self.result_types();
        let first_result = result_types.next().unwrap();
        first_result
    }

    pub fn expect_branch(&self) -> &Branch {
        if let Node::Branch(ref branch) = *self {
            branch
        } else {
            panic!("expected a branch but got {:?}", self);
        }
    }

    pub fn expect_leaf(&self) -> &Value {
        if let Node::Leaf(ref value) = *self {
            value
        } else {
            panic!("expected a leaf but got {:?}", self);
        }
    }
}

