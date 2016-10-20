use {OpCode, Branch, Value, Type};

use util;
use std;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct NodeId(pub util::Id);

#[derive(Clone,PartialEq,Eq)]
pub struct Node
{
    pub id: NodeId,
    pub kind: NodeKind,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum NodeKind
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
        Node {
            id: NodeId(util::Id::next()),
            kind: NodeKind::Branch(Branch {
                opcode: opcode,
                operands: operands.into_iter().collect(),
            }),
        }
    }

    /// Creates a new leaf node.
    pub fn leaf(value: Value) -> Self {
        Node {
            id: NodeId(util::Id::next()),
            kind: NodeKind::Leaf(value)
        }
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

    pub fn set(register_ref: Self, value: Self) -> Self {
        Self::branch(OpCode::Set, vec![register_ref, value])
    }

    /// Creates an sum of values node.
    pub fn add(addends: &[Self]) -> Self {
        Self::branch(OpCode::Add, addends.to_owned())
    }

    /// Creates a difference between values.
    pub fn sub(addends: &[Self]) -> Self {
        Self::branch(OpCode::Sub, addends.to_owned())
    }

    pub fn new_register_ref(ty: Type) -> Self {
        Self::leaf(Value::register_ref(util::Id::next(), 0, ty))
    }

    pub fn result_types(&self) -> ::std::vec::IntoIter<Type> {
        match self.kind {
            NodeKind::Branch(ref branch) => match branch.opcode {
                OpCode::Add |
                OpCode::Sub |
                OpCode::Mul |
                OpCode::Div |
                OpCode::Shl |
                OpCode::Shr => {
                    // FIXME: check that all types are the same.
                    vec![branch.operands[0].ty()]
                },
                OpCode::Ret |
                OpCode::Set => {
                    vec![Type::Nothing]
                }
                OpCode::Sext |
                OpCode::Zext => {
                    assert_eq!(branch.operands.len(), 2);
                    let bit_width = branch.operands.first().unwrap().expect_leaf().
                        expect_constant_integer().bit_width;

                    vec![Type::Integer { bit_width: bit_width as _ }]
                },
            },
            NodeKind::Leaf(ref value) => vec![value.ty()]
        }.into_iter()
    }

    // FIXME: get rid of this so we can support multiple
    // results
    pub fn ty(&self) -> Type {
        let mut result_types = self.result_types();
        let first_result = result_types.next().unwrap();
        first_result
    }

    /// Recursively map this node and all child nodes.
    pub fn recursive_map<F>(mut self, f: &mut F) -> Self
        where F: FnMut(Self) -> Self {
        self = f(self);

        match self.kind {
            NodeKind::Branch(mut branch) => {
                branch.operands = branch.operands.into_iter().map(|operand| {
                    operand.recursive_map(f)
                }).collect();

                Node { kind: NodeKind::Branch(branch), ..self }
            },
            NodeKind::Leaf(value) => Node { kind: NodeKind::Leaf(value), ..self },
        }
    }

    pub fn expect_branch(&self) -> &Branch {
        if let NodeKind::Branch(ref branch) = self.kind {
            branch
        } else {
            panic!("expected a branch but got {:?}", self);
        }
    }

    pub fn expect_leaf(&self) -> &Value {
        if let NodeKind::Leaf(ref value) = self.kind {
            value
        } else {
            panic!("expected a leaf but got {:?}", self);
        }
    }
}

impl std::fmt::Debug for Node
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.kind {
            NodeKind::Branch(ref b) => write!(fmt, "({:?})", b),
            NodeKind::Leaf(ref v)   => std::fmt::Debug::fmt(v, fmt),
        }
    }
}
