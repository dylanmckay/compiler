use {RegisterClass, Register};
use util;
use std;

/// Defines whether an operand is an input or an output (or both).
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Direction
{
    Input,
    Output,
    InputOutput,
}

/// Info about an operand.
// TODO:
#[derive(Clone,PartialEq,Eq)]
pub struct OperandInfo
{
    /// The value of the operand.
    pub value: Operand,
    /// Whether the operand is an input or an output.
    pub direction: Direction,
}

/// A machine operand.
#[derive(Clone,PartialEq,Eq)]
pub enum Operand
{
    /// An immediate value.
    Immediate { bit_width: u32, value: i64 },

    /// A machine register.
    Register(&'static Register),

    /// A virtual register.
    VirtualRegister {
        id: util::Id,
        class: &'static RegisterClass,
    },
}

impl OperandInfo
{
    pub fn new(value: Operand, direction: Direction) -> Self {
        OperandInfo {
            value: value,
            direction: direction,
        }
    }

    pub fn input(value: Operand) -> Self { Self::new(value, Direction::Input) }
    pub fn output(value: Operand) -> Self { Self::new(value, Direction::Output) }

    pub fn input_output(value: Operand) -> Self {
        Self::new(value, Direction::InputOutput)
    }
}

impl std::fmt::Debug for OperandInfo
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.value, fmt)
    }
}

impl std::fmt::Debug for Operand
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Operand::Immediate { bit_width, value } => {
                write!(fmt, "i{}:{}", bit_width, value)
            },
            Operand::Register(reg) => {
                write!(fmt, "{}", reg.name)
            },
            Operand::VirtualRegister { id, class } => {
                write!(fmt, "<{}:#{}>", class.name, id)
            },
        }
    }
}

