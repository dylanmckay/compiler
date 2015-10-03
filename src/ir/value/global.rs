
use ir::{self,Value};

#[derive(Clone,Debug)]
pub struct Global
{
    name: ir::Name,
    value: ir::Value,
}


impl Global
{
    pub fn new(name: ir::Name, value: ir::Value) -> Self {
        Global {
            name: name,
            value: value,
        }
    }

    pub fn name(&self) -> ir::Name { self.name.clone() }
    pub fn value(&self) -> ir::Value { self.value.clone() }
}
