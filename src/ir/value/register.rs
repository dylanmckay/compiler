
use ir;
use std;

/// A register.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Register
{
    name: ir::Name,
    ty: ir::Type,
}

impl Register
{
    /// Creates a new register.
    pub fn new(name: ir::Name, ty: ir::Type) -> Self {
        Register {
            name: name,
            ty: ty,
        }
    }

    /// Creates an unnamed register.
    pub fn unnamed(ty: ir::Type) -> Self {
        Register::new(ir::Name::Unnamed, ty)
    }

    pub fn name(&self) -> &ir::Name { &self.name }

    pub fn ty(&self) -> ir::Type {
        self.ty.clone()
    }
}

impl std::fmt::Display for Register
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{} %{}", self.ty, self.name)
    }
}

impl Into<ir::Value> for Register
{
    fn into(self) -> ir::Value {
        ir::Value::Register(self)
    }
}
