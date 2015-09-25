use ir::{self,Instruction,InstructionTrait,Value,ValueTrait};
use std::fmt;

#[derive(Clone,Debug)]
pub struct Sub
{
    ty:  Box<ir::Type>,
    lhs: Box<ir::Value>,
    rhs: Box<ir::Value>,
}

impl Sub
{
    pub fn new(ty: ir::Type, lhs: ir::Value, rhs: ir::Value) -> Self {
        Sub {
            ty: Box::new(ty),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

impl InstructionTrait for Sub { }

impl fmt::Display for Sub
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(fmt, "sub {} {}, {}", self.ty, self.lhs, self.rhs)
    }
}

impl_upcast!(Sub,Instruction);
