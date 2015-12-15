use OpCode;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Node
{
    opcode: OpCode,
    operands: Vec<>,
}
