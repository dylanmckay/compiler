
use ir;
use std::fmt;

/// A pointer.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Pointer
{
    to: Box<ir::Value>,
}

impl Pointer
{
    pub fn to(value: ir::Value) -> Self {
        Pointer {
            to: Box::new(value),
        }
    }

    pub fn deref(self) -> ir::Value {
        *self.to
    }

    pub fn ty(&self) -> ir::Type {
        use lang::Value;
        ir::Type::pointer(self.to.ty())
    }
}

impl fmt::Display for Pointer
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "&{}", self.to)
    }
}

impl ir::value::ValueTrait for Pointer { }

impl Into<ir::Value> for Pointer
{
    fn into(self) -> ir::Value {
        ir::Value::Pointer(self)
    }
}
