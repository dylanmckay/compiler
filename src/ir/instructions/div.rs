
use ir::{self,Instruction,InstructionTrait,Value,ValueTrait};
use std::fmt;

#[derive(Clone,Debug)]
pub struct Div
{
    ty:  Box<ir::Type>,
    lhs: Box<ir::Value>,
    rhs: Box<ir::Value>,
}

impl Div
{
    pub fn new(ty: ir::Type, lhs: ir::Value, rhs: ir::Value) -> Self {
        Div {
            ty: Box::new(ty),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

impl InstructionTrait for Div { }

impl fmt::Display for Div
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(fmt, "div {} {}, {}", self.ty, self.lhs, self.rhs)
    }
}

impl_upcast!(Div,Instruction);
