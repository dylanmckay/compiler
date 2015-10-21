
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

    pub fn subvalue(&self) -> Option<&ir::Value> {
        if let Some(ref val) = self.value {
            Some(val)
        } else {
            None
        }
    }

    pub fn map_subvalues<F>(mut self, mut f: F) -> Self
        where F: FnMut(Value) -> Value {

        let value = match self.value {
            Some(val) => Some(Box::new(f(*val.clone()))),
            None => self.value,
        };

        self.value = value;
        self
    }

    pub fn ty(&self) -> ir::Type { ir::Type::void() }
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
