
use ir::{self,types,Value,ValueTrait,Type};
use util;
use std::fmt;

#[derive(Clone,Debug)]
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
}

impl ir::value::ConstantTrait for Struct { }

impl ValueTrait for Struct
{
    fn ty(&self) -> Type {
        // Create the struct type from the types of the values.
        types::Struct::new(
            self.fields.iter().map(|ref f| f.ty())
        ).into()
    }
}

impl fmt::Display for Struct
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(fmt, "{{ {} }}", util::comma_separated_values(self.fields.iter()))
    }
}

impl Into<Value> for Struct
{
    fn into(self) -> Value {
        Value::Constant(self.into())
    }
}

impl Into<ir::value::Constant> for Struct {
    fn into(self) -> ir::value::Constant {
        ir::value::Constant::Struct(self)
    }
}

