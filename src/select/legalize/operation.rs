use legalize::Action;

use mir;

pub struct Operation
{
    pub opcode: mir::OpCode,
    pub result_types: Vec<mir::Type>,
    pub action: Action,
}

