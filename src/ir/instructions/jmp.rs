
use ir::{self,Instruction,InstructionTrait,Value,ValueTrait};
use util;
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

impl InstructionTrait for Jump { }

impl fmt::Display for Jump
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        let func = if let Value::Function(ref f) = *self.target {
            f
        } else {
            unreachable!(); // target must be function
        };

        try!("jump ".fmt(fmt));
        try!(util::fmt_comma_separated_values(func.signature.return_types.iter(), fmt));
        write!(fmt, " {}", func.name)
    }
}

impl_upcast!(Jump,Instruction);
