
use ir::{self,Instruction,InstructionTrait,Value,ValueTrait};
use std::fmt;

#[derive(Clone,Debug)]
pub struct Shr
{
    ty: Box<ir::Type>,
    value: Box<ir::Value>,
    amount: Box<ir::Value>,
}

impl Shr
{
    pub fn new(ty: ir::Type, value: ir::Value, amount: ir::Value) -> Self {
        Shr {
            ty: Box::new(ty),
            value: Box::new(value),
            amount: Box::new(amount),
        }
    }
}

impl InstructionTrait for Shr { }

impl fmt::Display for Shr
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(fmt, "shr {} {}, {}", self.ty, self.value, self.amount)
    }
}

impl_upcast!(Shr,Instruction);
