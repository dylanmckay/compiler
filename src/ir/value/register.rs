
use ir;
use std;

/// A register.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Register
{
    name: ir::Name,
    value: Box<ir::Value>,
}

impl Register
{
    /// Creates a new register.
    pub fn new(name: ir::Name, value: ir::Value) -> Self {
        Register {
            name: name,
            value: Box::new(value),
        }
    }

    /// Creates an unnamed register.
    pub fn unnamed(value: ir::Value) -> Self {
        Register::new(ir::Name::Unnamed, value)
    }

    pub fn name(&self) -> &ir::Name { &self.name }

    pub fn ty(&self) -> ir::Type {
        self.value.ty()
    }
}

impl std::fmt::Display for Register
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "%{}", self.name)
    }
}

impl Into<ir::Value> for Register
{
    fn into(self) -> ir::Value {
        ir::Value::Register(self)
    }
}
