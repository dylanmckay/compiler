use {Register, RegisterClass};
use std;

pub trait Instruction : std::fmt::Debug
{
    type Operand: Operand;
    type RegisterClass: RegisterClass;
    type Register: Register;

    fn operands_mut(&mut self) -> Vec<Box<Operand>>;
}

pub trait Operand : std::fmt::Debug
{
    fn is_virtual(&self) -> bool;
}

