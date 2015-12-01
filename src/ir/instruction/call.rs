
use ir::{self,Instruction,Value,Expression};

// TODO: allow passing values as arguments.

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

    pub fn target_id(&self) -> ::util::Id {
        if let ir::Expression::FunctionRef(ref r) = *self.target().expression() {
            r.function_id()
        } else {
            panic!("a call instruction must have a function reference as its target");
        }
    }

    pub fn ty(&self) -> ir::Type {
        self.target.ty()
    }
}

impl_instruction!(Call: target);

