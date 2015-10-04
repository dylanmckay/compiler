
use ir::{self,Instruction,Value,ValueTrait};
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
}

impl ValueTrait for Jump
{
    fn ty(&self) -> ir::Type { ir::Type::void() }
}

impl fmt::Display for Jump
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        use util;

        let func = if let Value::Function(ref f) = *self.target {
            f
        } else {
            unreachable!(); // target must be function
        };

        write!(fmt, "jump {} {}", util::comma_separated_values(func.signature.returns()),
                                  func.name)
    }
}

impl_instruction!(Jump: target);
