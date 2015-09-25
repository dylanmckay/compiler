
use ir::{self,Instruction,InstructionTrait,Value,ValueTrait};
use util;
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

impl InstructionTrait for Call { }

impl fmt::Display for Call
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        let func = if let Value::Function(ref f) = *self.target {
            f
        } else {
            unreachable!(); // target must be function
        };

        try!("call ".fmt(fmt));
        try!(util::fmt_comma_separated_values(func.signature.return_types.iter(), fmt));
        write!(fmt, " {}", func.name)
    }
}

impl_upcast!(Call,Instruction);
