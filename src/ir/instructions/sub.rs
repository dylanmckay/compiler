use ir::{self,Instruction,Value,ValueTrait};
use std::fmt;

#[derive(Clone,Debug)]
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

    pub fn terms(&self) -> (ir::Value,ir::Value) {
        (*self.lhs.clone(), *self.rhs.clone())
    }
}

impl ValueTrait for Sub
{
    fn ty(&self) -> ir::Type { self.ty.clone() }
}

impl fmt::Display for Sub
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(fmt, "sub {} {}, {}", self.ty, self.lhs, self.rhs)
    }
}

impl_lang_instruction!(Sub: lhs, rhs);
