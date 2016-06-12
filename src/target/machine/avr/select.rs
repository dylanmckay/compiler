use select;
use machine;
use mir;

use machine::avr::instruction;

pub fn selector() -> select::Selector<Box<machine::Instruction>> {
    select::Selector::new(Box::new(select_node))
}

pub fn select_node(node: &mir::Node) -> Option<Box<machine::Instruction>> {
    let branch = node.expect_branch();

    match branch.opcode {
        mir::OpCode::Ret => Some(Box::new(instruction::RET)),
        _ => unimplemented!(),
    }
}

