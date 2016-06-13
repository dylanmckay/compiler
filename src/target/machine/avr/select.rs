use {Pattern, PatternNode, PatternOperand};
use machine::avr::registers;
use machine;

use mir;
use select;

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

pub fn selector() -> select::Selector<Box<machine::Instruction>> {
    unimplemented!();
}

pub fn patterns() -> Vec<Pattern> {
    vec![
        pattern! {
            node!(Add,
                  operands!(
                      PatternOperand::Register(&registers::GPR8),
                      PatternOperand::Register(&registers::GPR8)
                  )
            )
        },
        pattern! { node!(Ret) },
    ]
}

