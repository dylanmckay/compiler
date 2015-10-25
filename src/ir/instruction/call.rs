
use ir::{self,Instruction,Value};

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
        self.target.ty()
    }
}

impl_instruction!(Call: target);

