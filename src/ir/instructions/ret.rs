
use ir::{self,Instruction,Value,ValueTrait};
use std::fmt;

#[derive(Clone,Debug)]
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

impl ValueTrait for Return
{
    fn ty(&self) -> ir::Type { unreachable!() }
}

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

impl_lang_instruction!(Return);
