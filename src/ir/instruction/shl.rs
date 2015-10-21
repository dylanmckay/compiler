
use ir::{self,Instruction,Value};
use std::fmt;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Shl
{
    value: Box<ir::Value>,
    amount: Box<ir::Value>,
}

impl Shl
{
    pub fn new(value: ir::Value, amount: ir::Value) -> Self {
        assert!(value.ty() == amount.ty());

        Shl {
            value: Box::new(value),
            amount: Box::new(amount),
        }
    }

    pub fn ty(&self) -> ir::Type { self.value.ty() }
}

impl fmt::Display for Shl
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(fmt, "shl {}, {}", self.value, self.amount)
    }
}

impl_instruction!(Shl: value, amount);
