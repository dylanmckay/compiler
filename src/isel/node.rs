
use num::BigInt;
use num::bigint::ToBigInt;

// TODO: proper float support
// TODO: factor out ir/value/literal/integer,rs so we can use it too.

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Operation
{
    Add,
    Sub,
    Mul,
    Div,
    Shl,
    Shr,
    Call,
    Return,
    Jump,
}

/// A value.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Value
{
    Integer(BigInt),
    Float(Vec<u8>),
}

/// A node.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Node
{
    Operation(Operation, Vec<Node>),
    Value(Value),
}

impl Node
{
    /// Creates an integer value node.
    pub fn integer<I>(value: I) -> Self
        where I: ToBigInt {

        // TODO: handle errors gracefully

        let bigint = value.to_bigint().expect("value cannot be converted into an integer");
        Node::Value(Value::Integer(bigint))
    }

    /// Creates an operation node.
    pub fn operation<I>(operation: Operation,
                        children: I) -> Self
        where I: Iterator<Item=Node> {

        Node::Operation(operation, children.collect())
    }
}

