
use ir::{self,Instruction,Value};
use std::fmt;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Mul
{
    pub lhs: Box<ir::Value>,
    pub rhs: Box<ir::Value>,
}

impl Mul
{
    pub fn new(lhs: ir::Value, rhs: ir::Value) -> Self {
        Mul {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn ty(&self) -> ir::Type { self.lhs.ty() }
}

impl fmt::Display for Mul
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(fmt, "mul {}, {}", self.lhs, self.rhs)
    }
}

impl_instruction!(Mul: lhs, rhs);
