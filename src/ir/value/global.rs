
use ir::{self,Value};
use std;

#[derive(Clone,Debug)]
pub struct Global
{
    name: ir::Name,
    value: Box<ir::Value>,
}

impl Global
{
    pub fn new(name: ir::Name, value: ir::Value) -> Self {
        Global {
            name: name,
            value: Box::new(value),
        }
    }

    pub fn name(&self) -> ir::Name { self.name.clone() }
    pub fn value(&self) -> ir::Value { *self.value.clone() }
}

impl std::fmt::Display for Global
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.name.fmt(fmt)
    }
}

impl ir::value::ValueTrait for Global
{
    fn ty(&self) -> ir::Type {
        ir::Type::pointer(self.value.ty())
    }
}

impl Into<ir::Value> for Global
{
    fn into(self) -> ir::Value {
        ir::Value::Global(self)
    }
}
