
use ir::{self,Instruction,InstructionTrait,Value,ValueTrait};
use std::fmt;

#[derive(Clone,Debug)]
pub struct Mul
{
    ty:  Box<ir::Type>,
    lhs: Box<ir::Value>,
    rhs: Box<ir::Value>,
}

impl Mul
{
    pub fn new(ty: ir::Type, lhs: ir::Value, rhs: ir::Value) -> Self {
        Mul {
            ty: Box::new(ty),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

impl InstructionTrait for Mul { }

impl fmt::Display for Mul
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(fmt, "mul {} {}, {}", self.ty, self.lhs, self.rhs)
    }
}

impl_upcast!(Mul,Instruction);
