use {Instruction,Value,Expression,Type};

// TODO: allow passing values as arguments.

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Call
{
    target: Box<Value>,
}

impl Call
{
    pub fn new(target: Value) -> Self {
        Call {
            target: Box::new(target),
        }
    }

    pub fn target(&self) -> &Value {
        &self.target
    }

    pub fn target_id(&self) -> ::util::Id {
        if let Expression::FunctionRef(ref r) = *self.target().expression() {
            r.function_id()
        } else {
            panic!("a call instruction must have a function reference as its target");
        }
    }

    pub fn ty(&self) -> Type {
        self.target.ty()
    }
}

impl_instruction!(Call: target);

