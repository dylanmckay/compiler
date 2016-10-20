use {OpCode, Node, ValueInfo};

use std;

#[derive(Clone,PartialEq,Eq)]
pub struct Branch {
    pub opcode: OpCode,
    pub operands: Vec<Node>,
}

impl Branch
{
    pub fn value_infos(&self) -> Vec<ValueInfo> {
        self.opcode.value_infos(&self.operands)
    }
}

impl std::fmt::Debug for Branch
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let operands: Vec<_> = self.operands.iter().map(|op| format!("{:?}", op)).collect();

        write!(fmt, "{} {}", self.opcode.mnemonic(), operands.join(", "))
    }
}

