use ir::{self,Instruction,Value};
use std::fmt;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Sub
{
    lhs: Box<ir::Value>,
    rhs: Box<ir::Value>,
}

impl Sub
{
    pub fn new(lhs: ir::Value, rhs: ir::Value) -> Self {
        assert!(lhs.ty() == rhs.ty());

        Sub {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn ty(&self) -> ir::Type { self.lhs.ty() }
}

impl fmt::Display for Sub
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(fmt, "sub {}, {}", self.lhs, self.rhs)
    }
}

impl_instruction!(Sub: lhs, rhs);
