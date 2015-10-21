
use ir::{self,types,Value,ValueTrait,Type};
use util;
use std;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Struct
{
    fields: Vec<Value>,
}

impl Struct
{
    pub fn new(fields: Vec<Value>) -> Self {

        Struct {
            fields: fields,
        }
    }

    pub fn fields<'a>(&'a self) -> std::slice::Iter<'a,Value> {
        self.fields.iter()
    }

    pub fn ty(&self) -> Type {
        use lang::Value;

        // Create the struct type from the types of the values.
        types::Struct::new(
            self.fields.iter().map(|ref f| f.ty())
        ).into()
    }
}

impl ir::value::LiteralTrait for Struct { }

impl ValueTrait for Struct { }

impl Into<Value> for Struct
{
    fn into(self) -> Value {
        Value::Literal(self.into())
    }
}

impl Into<ir::value::Literal> for Struct {
    fn into(self) -> ir::value::Literal {
        ir::value::Literal::Struct(self)
    }
}

