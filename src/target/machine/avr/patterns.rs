use {Pattern, PatternNode, PatternOperand};

use mir;

macro_rules! pattern {
    ($node:expr) => { Pattern { root: $node } }
}

macro_rules! node {
    ($opcode:ident, $operands:expr) => {
        PatternNode {
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

pub fn patterns() -> Vec<Pattern> {
    vec![
        pattern! {
            node!(Add,
                  operands!(
                      PatternOperand::Immediate { width: 8 }
                  )
            )
        },
        pattern! { node!(Ret) },
    ]
}

