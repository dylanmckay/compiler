
use ir::{self,Instruction,Value};
use std::fmt;

#[derive(Clone,Debug,PartialEq,Eq)]
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

    pub fn subvalues(&self) -> Vec<&Value> {
        if let Some(ref value) = self.value {
            vec![value]
        } else {
            vec![]
        }
    }

    pub fn map_subvalues<F>(mut self, mut f: F) -> Value
        where F: FnMut(Value) -> Value {

        let value = match self.value {
            Some(val) => Some(Box::new(f(*val.clone()))),
            None => self.value,
        };

        self.value = value;
        self.into()
    }

    pub fn ty(&self) -> ir::Type { ir::Type::void() }
}

impl fmt::Display for Return
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        try!("ret ".fmt(fmt));
        
        match self.value {
            Some(ref val) => { write!(fmt, "{}", val) },
            None =>          { write!(fmt, "void")  },
        }
    }
}

impl Into<Instruction> for Return
{
    fn into(self) -> Instruction {
        ir::Instruction::Return(self)
    }
}

impl Into<Value> for Return
{
    fn into(self) -> Value {
        ir::Value::Instruction(self.into())
    }
}
