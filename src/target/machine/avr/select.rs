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

/// An instruction which takes a destination and source GPR8.
macro_rules! inst_rdrr {
    ($opcode:ident) => {
        pattern! {
            node!(Set,
                operands!(
                    select::PatternOperand::Value(PatternOperand::Register(&registers::GPR8)),
                    select::PatternOperand::Node(Box::new(node!($opcode,
                        operands!(
                            select::PatternOperand::Value(PatternOperand::Register(&registers::GPR8)),
                            select::PatternOperand::Value(PatternOperand::Register(&registers::GPR8))
                        )
                    )))
                )
            )
        }
    }
}

/// An instruction which takes a GPR8 and an 8-bit immediate.
macro_rules! inst_rdi {
    ($opcode:ident) => {
        pattern! {
            node!(Set,
                operands!(
                    select::PatternOperand::Value(PatternOperand::Register(&registers::GPR8)),
                    select::PatternOperand::Node(Box::new(node!($opcode,
                          operands!(
                              select::PatternOperand::Value(PatternOperand::Register(&registers::GPR8)),
                              select::PatternOperand::Value(PatternOperand::Immediate { width: 8 })
                          )
                    )))
                )
            )
        }
    }
}

pub fn selector() -> machine::Selector {
    machine::Selector::new(self::patterns())
}

pub fn patterns() -> Vec<Pattern> {
    vec![
        inst_rdi!(Add),  // ADDRdK

        inst_rdrr!(Add), // ADDRdRr
        inst_rdrr!(Sub), // SUBRdRr

        pattern! { node!(Ret) },

        pattern! {
            node!(Ret,
                  operands!(
                      select::PatternOperand::Node(Box::new(node!(Add,
                          operands!(
                              select::PatternOperand::Value(PatternOperand::Register(&registers::GPR8)),
                              select::PatternOperand::Value(PatternOperand::Register(&registers::GPR8))
                          )
                      )))
                  )
            )
        }

    ]
}

