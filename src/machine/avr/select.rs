use {Pattern, PatternNode, PatternOperand, Selector, Instruction};
use avr::{registers, instruction};

use select;

use mir;
use std;

fn bar(node: &mir::Node) -> Box<Instruction> {
    Box::new(instruction::RET)
}

macro_rules! pattern {
    ($ty:ident, $node:expr) => {
        Pattern {
            root: $node,
            factory: instruction::$ty::from_pattern,
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
    ($ty:ident, $opcode:ident) => {
        pattern!($ty, {
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
        })
    }
}

/// An instruction which takes a GPR8 and an 8-bit immediate.
macro_rules! inst_rdi {
    ($ty:ident, $opcode:ident) => {
        pattern!($ty, {
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
        })
    }
}

pub fn selector() -> Selector {
    Selector::new(self::patterns())
}

pub fn patterns() -> Vec<Pattern> {
    vec![
        // inst_rdi!(ADD, Add),  // ADDRdK

        inst_rdrr!(ADDRdRr, Add), // ADDRdRr
        inst_rdrr!(ADDRdRr, Sub), // SUBRdRr

        pattern!(RET, { node!(Ret) }),

        // LDI Rd, K
        pattern!(ADDRdRr, {
            node!(Set,
                  operands!(
                      select::PatternOperand::Value(PatternOperand::Register(&registers::GPR8)),
                      select::PatternOperand::Value(PatternOperand::Immediate { width: 8 })
                  )
            )
        }),

        pattern!(RET, {
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
        })

    ]
}

