use {Pattern, PatternNode, PatternOperand, Selector};
use avr::registers;

use select;

use mir;
use std;

fn bar(node: &mir::Node) -> mir::Node {
    node.clone()
}

macro_rules! pattern {
    ($node:expr) => {
        Pattern {
            root: $node,
            factory: unsafe { std::mem::transmute(&bar) },
        }
    }
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

pub fn selector() -> Selector {
    Selector::new(self::patterns())
}

pub fn patterns() -> Vec<Pattern> {
    vec![
        inst_rdi!(Add),  // ADDRdK

        inst_rdrr!(Add), // ADDRdRr
        inst_rdrr!(Sub), // SUBRdRr

        pattern! { node!(Ret) },

        // LDI Rd, K
        pattern! {
            node!(Set,
                  operands!(
                      select::PatternOperand::Value(PatternOperand::Register(&registers::GPR8)),
                      select::PatternOperand::Value(PatternOperand::Immediate { width: 8 })
                  )
            )
        },

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

