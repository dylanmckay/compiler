
use ir::{self,Instruction,Value};
use std::fmt;

#[derive(Clone,Debug)]
pub struct Mul
{
    pub ty:  ir::Type,
    pub lhs: Box<ir::Value>,
    pub rhs: Box<ir::Value>,
}

impl Mul
{
    pub fn new(ty: ir::Type, lhs: ir::Value, rhs: ir::Value) -> Self {
        Mul {
            ty: ty,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn ty(&self) -> ir::Type { self.ty.clone() }
}

impl fmt::Display for Mul
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(fmt, "mul {} {}, {}", self.ty, self.lhs, self.rhs)
    }
}

impl_instruction!(Mul: lhs, rhs);
