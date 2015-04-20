
use std::fmt;

#[derive(Debug)]
pub enum OperandKind
{
    Register(Register),
}

pub trait Operand : fmt::Debug
{

}

#[derive(Copy,Clone,Debug)]
pub struct Register
{
    num: u8,
}

impl Register
{
    pub fn number(&self) -> u8 { self.num }
}

impl Operand for Register { }
