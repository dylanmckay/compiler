use OpCode;
use Value;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Node<V = Value>
{
    opcode: OpCode,
    operands: Vec<V>,
}

macro_rules! define_ctor {
    ($name:ident, $opcode:expr) => {
        pub fn $name<I>(operands: I) -> Self
            where I: IntoIterator<Item=Value> {
            Node::new($opcode, operands)
        }
    }
}

impl<V> Node<V>
{
    pub fn new<I>(opcode: OpCode,
                  operands: I) -> Self
        where I: IntoIterator<Item=V> {
        Node {
            opcode: opcode,
            operands: operands.into_iter().collect(),
        }
    }
}
