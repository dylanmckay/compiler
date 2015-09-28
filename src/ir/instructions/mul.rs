
use ir::{self,Instruction,InstructionTrait,Value,ValueTrait};
use std::fmt;

#[derive(Clone,Debug)]
pub struct Mul
{
    pub ty:  Box<ir::Type>,
    pub lhs: Box<ir::Value>,
    pub rhs: Box<ir::Value>,
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

    pub fn ty(&self) -> &ir::Type {
        &self.ty
    }

    pub fn multiplicands(&self) -> (Value,Value) {
        (*self.lhs.clone(), *self.rhs.clone())
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
