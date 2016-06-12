use select;
use mir;

macro_rules! pattern {
    ($node:expr) => { select::Pattern { root: $node } }
}

macro_rules! node {
    ($opcode:ident, $operands:expr) => {
        select::PatternNode {
            opcode: mir::OpCode::$opcode,
            operands: $operands,
        }
    };

    ($opcode:ident) => {
        node!($opcode, vec![])
    }

}

macro_rules! operands {
    ($($operand:expr),*) => {
        vec![$( $operand ),+]
    }
}

pub fn patterns() -> Vec<select::Pattern> {
    vec![
        pattern! {
            node!(Add,
                  operands!(
                      select::PatternOperand::Immediate { width: 8 }
                  )
            )
        },
        pattern! { node!(Ret) },
    ]
}

