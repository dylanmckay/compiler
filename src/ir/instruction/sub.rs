use ir::{self,Instruction,Value};
use std::fmt;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Sub
{
    ty:  ir::Type,
    lhs: Box<ir::Value>,
    rhs: Box<ir::Value>,
}

impl Sub
{
    pub fn new(ty: ir::Type, lhs: ir::Value, rhs: ir::Value) -> Self {
        Sub {
            ty: ty,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn ty(&self) -> ir::Type { self.ty.clone() }
}

impl fmt::Display for Sub
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(fmt, "sub {} {}, {}", self.ty, self.lhs, self.rhs)
    }
}

impl_instruction!(Sub: lhs, rhs);
