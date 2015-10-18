
use ir;
use util;

use std;

#[derive(Clone,Debug)]
pub struct Global
{
    id: util::Id,

    name: String,
    value: Box<ir::Value>,
}

impl Global
{
    pub fn new(name: String, value: ir::Value) -> Self {
        Global {
            id: util::Id::unspecified(),

            name: name,
            value: Box::new(value),
        }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn value(&self) -> ir::Value { *self.value.clone() }
    pub fn ty(&self) -> ir::Type {
        use ir::ValueTrait;
        self.value.ty()
    }

    /// Gets a reference to the global.
    pub fn reference(&self) -> ir::Value {
        ir::Value::global_ref(self)
    }

    /// Gets the ID of the global.
    ///
    /// The ID is guaranteed to be unique for each module.
    pub fn id(&self) -> util::Id { self.id }
}

impl util::id::Identifiable for Global
{
    fn set_id(&mut self, id: util::Id) {
        self.id = id;
    }
}

impl std::fmt::Display for Global
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.name.fmt(fmt)
    }
}

