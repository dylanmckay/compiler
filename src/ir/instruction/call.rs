
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

    pub fn ty(&self) -> ir::Type {
        ir::Type::void()
    }
}

impl fmt::Display for Call
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        use util;

        let func = if let Value::FunctionRef(ref f) = *self.target {
            f
        } else {
            unreachable!(); // target must be function
        };
        write!(fmt, "call {} {}",
               util::comma_separated_values(func.signature().returns()),
               func.name())
    }
}

impl_instruction!(Call: target);
