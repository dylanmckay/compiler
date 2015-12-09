use {Value,Type,Name};
use util;

/// A register.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Register
{
    id: util::Id,

    name: Name,
    value: Box<Value>,
}

impl Register
{
    /// Creates a new register.
    pub fn new(name: Name, value: Value) -> Self {
        Register {
            id: util::Id::next(),

            name: name,
            value: Box::new(value),
        }
    }

    /// Creates an unnamed register.
    pub fn unnamed(value: Value) -> Self {
        Register::new(Name::Unnamed, value)
    }

    pub fn name(&self) -> &Name { &self.name }

    pub fn subvalue(&self) -> &Value {
        &self.value
    }

    pub fn ty(&self) -> Type {
        // the register itself has no type.
        // only references to the register have one.
        Type::void()
    }
}

impl_expression!(Register);
