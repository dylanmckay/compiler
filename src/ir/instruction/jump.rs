
use ir::{self,Instruction,Value};
use std::fmt;

#[derive(Clone,Debug)]
pub struct Jump
{
    target: Box<ir::Value>,
}

impl Jump
{
    pub fn new(target: ir::Value) -> Self {
        Jump {
            target: Box::new(target),
        }
    }

    pub fn ty(&self) -> ir::Type { ir::Type::void() }
}

impl fmt::Display for Jump
{
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

impl_instruction!(Jump: target);
