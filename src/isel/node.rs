
use num::BigInt;
use num::bigint::ToBigInt;
use lang;
use std::fmt;

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

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct TempType;

impl lang::Type for TempType { }

impl lang::Value for Node
{
    // temporary
    type Type = TempType;

    fn subvalues(&self) -> Vec<Self> {
        unimplemented!();
    }

    fn map_subvalues<F>(self, f: F) -> Self
        where F: FnMut(Self) -> Self {
        unimplemented!();
    }

    fn ty(&self) -> TempType {
        unimplemented!();
    }
}

impl fmt::Display for TempType
{
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

impl fmt::Display for Node
{
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

