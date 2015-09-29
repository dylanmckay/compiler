
use ir::{self,Instruction,Value,ValueTrait};
use std::fmt;

#[derive(Clone,Debug)]
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

    pub fn operands(&self) -> (ir::Value,ir::Value) {
        (*self.value.clone(), *self.amount.clone())
    }
}

impl ValueTrait for Shr
{
    fn ty(&self) -> ir::Type { self.ty.clone() }
}

impl fmt::Display for Shr
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(fmt, "shr {} {}, {}", self.ty, self.value, self.amount)
    }
}

impl_lang_instruction!(Shr: value, amount);
