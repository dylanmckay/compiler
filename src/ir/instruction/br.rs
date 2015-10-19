
use ir::{self,Instruction,Value};
use std::fmt;

#[derive(Clone,Debug)]
pub struct Break
{
    target: Box<ir::Value>,
}

impl Break
{
    pub fn new(target: ir::Value) -> Self {
        Break {
            target: Box::new(target),
        }
    }

    pub fn ty(&self) -> ir::Type { ir::Type::void() }
}

impl fmt::Display for Break
{
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

impl_instruction!(Break: target);
