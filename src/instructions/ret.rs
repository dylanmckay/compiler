
use ir;
use ir::{Instruction,InstructionTrait,Value,ValueTrait};
use std::fmt;

#[derive(Clone)]
pub struct Return
{
    value: Option<Box<ir::Value>>,
}

impl Return
{
    pub fn new(value: Option<ir::Value>) -> Return {
        Return {
            value: value.map(|v| Box::new(v)),
        }
    }

    pub fn value(value: ir::Value) -> Return {
        Return::new(Some(value))
    }

    pub fn void() -> Return {
        Return::new(None)
    }
}

impl InstructionTrait for Return { }

impl fmt::Display for Return
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        try!("ret ".fmt(fmt));
        
        match self.value {
            Some(ref val) => { write!(fmt, "{} {}", val.ty(), val) },
            None =>          { write!(fmt, "void")  },
        }
    }
}

impl_upcast!(Return,Instruction);
