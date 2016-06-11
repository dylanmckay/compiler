use select;
use mir;

pub fn selector() -> select::Selector<()> {
    select::Selector::new(Box::new(select_node))
}

pub fn select_node(node: &mir::Node) -> Option<()> {
    let branch = node.expect_branch();

    match branch.opcode {
        mir::OpCode::Ret => panic!("woot"),
        _ => unimplemented!(),
    }
}

