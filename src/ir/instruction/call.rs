
use ir::{self,Instruction,Value};
use std::fmt;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Call
{
    target: Box<ir::Value>,
}

impl Call
{
    pub fn new(target: ir::Value) -> Self {
        Call {
            target: Box::new(target),
        }
    }

    pub fn target(&self) -> &ir::Value {
        &self.target
    }

    pub fn ty(&self) -> ir::Type {
        ir::Type::void()
    }
}

impl_instruction!(Call: target);

