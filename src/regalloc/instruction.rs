use {Register, RegisterClass};

use util;
use std;

pub trait Instruction : std::fmt::Debug
{
    type Operand: Operand;

    fn operands_mut(&mut self) -> Vec<&mut Self::Operand>;
}

pub trait Operand : std::fmt::Debug
{
    type Register: Register;
    type RegisterClass: RegisterClass;

    fn is_virtual(&self) -> bool;
    fn virtual_register_id(&self) -> util::Id;

    fn register_class(&self) -> Self::RegisterClass;

    fn allocate(&mut self, register: Self::Register);
}

