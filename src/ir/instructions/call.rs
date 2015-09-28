
use ir::{self,Instruction,Value};
use std::fmt;

#[derive(Clone,Debug)]
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
}

impl ir::ValueTrait for Call
{
    fn ty(&self) -> ir::Type {
        unreachable!();
    }
}

impl fmt::Display for Call
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        use util;

        let func = if let Value::Function(ref f) = *self.target {
            f
        } else {
            unreachable!(); // target must be function
        };

        write!(fmt, "call {} {}", util::comma_separated_values(func.signature.return_types.iter()),
                                  func.name)
    }
}

impl_lang_instruction!(Call: target);
