
use ir::{self,Instruction,Value};
use std::fmt;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Shr
{
    value: Box<ir::Value>,
    amount: Box<ir::Value>,
}

impl Shr
{
    pub fn new(value: ir::Value, amount: ir::Value) -> Self {
        assert!(value.ty() == amount.ty());

        Shr {
            value: Box::new(value),
            amount: Box::new(amount),
        }
    }

    pub fn ty(&self) -> ir::Type { self.value.ty() }
}

impl fmt::Display for Shr
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(fmt, "shr {}, {}", self.value, self.amount)
    }
}

impl_instruction!(Shr: value, amount);
