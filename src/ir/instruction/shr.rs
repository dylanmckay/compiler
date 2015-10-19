
use ir::{self,Instruction,Value};
use std::fmt;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Shr
{
    ty: ir::Type,
    value: Box<ir::Value>,
    amount: Box<ir::Value>,
}

impl Shr
{
    pub fn new(ty: ir::Type, value: ir::Value, amount: ir::Value) -> Self {
        Shr {
            ty: ty,
            value: Box::new(value),
            amount: Box::new(amount),
        }
    }

    pub fn ty(&self) -> ir::Type { self.ty.clone() }
}

impl fmt::Display for Shr
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(fmt, "shr {} {}, {}", self.ty, self.value, self.amount)
    }
}

impl_instruction!(Shr: value, amount);
