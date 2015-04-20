
use ir::{self,Instruction,InstructionTrait,Value,ValueTrait};
use std::fmt;

#[derive(Clone)]
pub struct Add
{
    ty:  Box<ir::Type>,
    lhs: Box<ir::Value>,
    rhs: Box<ir::Value>,
}

impl Add
{
    pub fn new(ty: ir::Type, lhs: ir::Value, rhs: ir::Value) -> Add {
        Add {
            ty: Box::new(ty),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

impl InstructionTrait for Add { }

impl fmt::Display for Add
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(fmt, "add {} {}, {}", self.ty, self.lhs, self.rhs)
    }
}

impl_upcast!(Add,Instruction);
