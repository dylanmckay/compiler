
use ir;
use util;

/// A register.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Register
{
    id: util::Id,

    name: ir::Name,
    value: Box<ir::Value>,
}

impl Register
{
    /// Creates a new register.
    pub fn new(name: ir::Name, value: ir::Value) -> Self {
        Register {
            id: util::Id::next(),

            name: name,
            value: Box::new(value),
        }
    }

    /// Creates an unnamed register.
    pub fn unnamed(value: ir::Value) -> Self {
        Register::new(ir::Name::Unnamed, value)
    }

    pub fn name(&self) -> &ir::Name { &self.name }

    pub fn subvalue(&self) -> &ir::Value {
        &self.value
    }

    pub fn ty(&self) -> ir::Type {
        // the register itself has no type.
        // only references to the register have one.
        ir::Type::void()
    }
}

impl util::Identifiable for Register
{
    fn get_id(&self) -> util::Id {
        self.id
    }
}

impl Into<ir::Expression> for Register
{
    fn into(self) -> ir::Expression {
        ir::Expression::Register(self)
    }
}
