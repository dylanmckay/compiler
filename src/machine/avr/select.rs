use {Pattern, PatternNode, PatternOperand, Selector};
use avr::{registers, instruction};

use select;
use mir;
use util;

macro_rules! pattern {
    ($ty:ident, $node:expr) => {
        pattern!($ty, $node, Vec::new());
    };

    ($ty:ident, $node:expr, $constraints:expr) => {
        Pattern {
            root: $node,
            factory: instruction::$ty::from_pattern,
        }
    };
}

macro_rules! node {
    ($opcode:ident, $operands:expr) => {
        PatternNode {
            id: util::Id::next(),
            opcode: mir::OpCode::$opcode,
            operands: $operands,
        }
    };

    ($opcode:ident) => {
        node!($opcode, vec![])
    }

}

macro_rules! value {
    ($name:expr => $value:expr) => {
        select::PatternOperand::Value {
            name: $name.to_string(),
            value: $value,
        }
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
        {
            pattern!($ty, {
                node!(Set,
                      operands!(
                          value!("rd" => PatternOperand::register(&registers::GPR8)),
                          select::PatternOperand::Node(Box::new(node!($opcode,
                              operands!(
                                  value!("rd" => PatternOperand::register(&registers::GPR8)),
                                  value!("rr" => PatternOperand::register(&registers::GPR8))
                              )
                          )))


                      )
                )
            })
        }
    }
}

/// An instruction which takes a GPR8 and an 8-bit immediate.
macro_rules! inst_rdi {
    ($ty:ident, $opcode:ident) => {
        {
            pattern!($ty, {
                node!(Set,
                    operands!(
                        value!("rd" => PatternOperand::register(&registers::GPR8)),
                        select::PatternOperand::Node(Box::new(node!($opcode,
                              operands!(
                                  value!("rd" => PatternOperand::register(&registers::GPR8)),
                                  value!("i" => PatternOperand::Immediate { width: 8 })
                              )
                        )))
                    )
                )
            })
        }
    }
}

/// An instruction which takes a DREGS and an 16-bit immediate.
macro_rules! inst_wide_rdi {
    ($ty:ident, $opcode:ident) => {
        {
            pattern!($ty, {
                node!(Set,
                    operands!(
                        value!("rd" => PatternOperand::register(&registers::IWREGS)),
                        select::PatternOperand::Node(Box::new(node!($opcode,
                              operands!(
                                  value!("rd" => PatternOperand::register(&registers::IWREGS)),
                                  value!("i" => PatternOperand::Immediate { width: 8 })
                              )
                        )))
                    )
                )
            })
        }
    }
}

pub fn selector() -> Selector {
    Selector::new(self::patterns())
}

pub fn patterns() -> Vec<Pattern> {
    vec![
        inst_wide_rdi!(ADIWRdK, Add),
        inst_rdi!(SUBIRdK, Sub),

        inst_rdrr!(ADDRdRr, Add),
        inst_rdrr!(SUBRdRr, Sub),

        pattern!(RET, { node!(Ret) }),

        pattern!(LDIRdK, {
            node!(Set,
                  operands!(
                      value!("rd" => PatternOperand::register(&registers::GPR8hi)),
                      value!("k"  => PatternOperand::Immediate { width: 8 })
                  )
            )
        }),
    ]
}

