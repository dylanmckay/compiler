use {Pattern, PatternNode, PatternOperand};
use machine::avr::registers;

use machine;
use select;

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

pub fn selector() -> machine::Selector {
    machine::Selector::new(self::patterns())
}

pub fn patterns() -> Vec<Pattern> {
    vec![
        pattern! {
            node!(Add,
                  operands!(
                      select::PatternOperand::Value(PatternOperand::Register(&registers::GPR8)),
                      select::PatternOperand::Value(PatternOperand::Register(&registers::GPR8))
                  )
            )
        },
        pattern! { node!(Ret) },
    ]
}

