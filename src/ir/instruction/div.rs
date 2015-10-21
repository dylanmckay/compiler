
use ir::{self,Instruction,Value};
use std::fmt;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Div
{
    lhs: Box<ir::Value>,
    rhs: Box<ir::Value>,
}

impl Div
{
    pub fn new(lhs: ir::Value, rhs: ir::Value) -> Self {
        use lang::Value;
        assert!(lhs.ty() == rhs.ty());

        Div {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn ty(&self) -> ir::Type { self.lhs.ty() }
}

impl_instruction!(Div: lhs, rhs);

