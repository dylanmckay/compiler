
use ir::{self,Instruction,Value};
use std::fmt;

#[derive(Clone,Debug)]
pub struct Div
{
    ty:  ir::Type,
    lhs: Box<ir::Value>,
    rhs: Box<ir::Value>,
}

impl Div
{
    pub fn new(ty: ir::Type, lhs: ir::Value, rhs: ir::Value) -> Self {
        Div {
            ty: ty,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn ty(&self) -> ir::Type { self.ty.clone() }
}

impl fmt::Display for Div
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(fmt, "div {} {}, {}", self.ty, self.lhs, self.rhs)
    }
}

impl_instruction!(Div: lhs, rhs);
