use ir::{self,Instruction,Value,ValueTrait};
use std::fmt;

#[derive(Clone,Debug)]
pub struct Add
{
    ty:  ir::Type,
    lhs: Box<ir::Value>,
    rhs: Box<ir::Value>,
}

impl Add
{
    pub fn new(ty: ir::Type, lhs: ir::Value, rhs: ir::Value) -> Self {
        Add {
            ty: ty,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn terms(&self) -> (ir::Value,ir::Value) {
        (*self.lhs.clone(), *self.rhs.clone())
    }
}

impl ValueTrait for Add
{
    fn ty(&self) -> ir::Type {
        self.ty.clone()
    }
}

impl fmt::Display for Add
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(fmt, "add {} {}, {}", self.ty, self.lhs, self.rhs)
    }
}

impl_lang_instruction!(Add: lhs, rhs);
