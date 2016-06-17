use std;

pub trait Instruction : std::fmt::Debug
{
    type Operand: Operand;

    fn operands_mut(&mut self) -> Vec<Box<Operand>>;
}

pub trait Operand : std::fmt::Debug
{
    fn is_virtual(&self) -> bool;
}

