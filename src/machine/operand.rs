use {RegisterClass, Register};

use regalloc;
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

    Regalloc(regalloc::Operand<Operand>),
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

impl Operand
{
    pub fn is_register(&self) -> bool {
        match *self {
            Operand::Immediate { .. } => false,
            Operand::Regalloc(ref a) => match *a {
                regalloc::Operand::VirtualRegister { .. } => true,
                regalloc::Operand::PhysicalRegister(..) => true,
            }
        }
    }
}

impl std::fmt::Debug for OperandInfo
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.value, fmt)
    }
}

impl regalloc::TargetOperand for Operand
{
    type Register = &'static Register;
    type RegisterClass = &'static RegisterClass;

    fn is_virtual(&self) -> bool {
        if let Operand::Regalloc(regalloc::Operand::VirtualRegister { .. }) = *self { true } else { false }
    }

    fn virtual_register_id(&self) -> util::Id {
        if let Operand::Regalloc(regalloc::Operand::VirtualRegister { id, .. }) = *self {
            id
        } else {
            panic!("operand is not a register");
        }
    }

    fn register_class(&self) -> &'static RegisterClass {
        match *self {
            Operand::Regalloc(ref r) => match *r {
                regalloc::Operand::PhysicalRegister(ref _r) => unimplemented!(),
                regalloc::Operand::VirtualRegister { class, .. } => class,
            },
            _ => panic!("operand is not a register"),
        }
    }

    fn allocate(&mut self, register: regalloc::Operand<Self>) {
        *self = Operand::Regalloc(register);
    }
}

impl std::fmt::Debug for Operand
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Operand::Immediate { value, .. } => {
                write!(fmt, "{}", value)
            },
            Operand::Regalloc(ref r) => match *r {
                regalloc::Operand::PhysicalRegister(reg) => {
                    write!(fmt, "{}", reg.name)
                },
                regalloc::Operand::VirtualRegister { id, class } => {
                    write!(fmt, "<{}:#{}>", class.name, id)
                },
            }
        }
    }
}

