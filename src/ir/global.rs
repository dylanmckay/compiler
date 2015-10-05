
use ir;
use util;

use std;

#[derive(Clone,Debug)]
pub struct Global
{
    id: util::Id,

    name: ir::Name,
    value: Box<ir::Value>,
}

impl Global
{
    pub fn new(name: ir::Name, value: ir::Value) -> Self {
        Global {
            id: util::Id::unspecified(),

            name: name,
            value: Box::new(value),
        }
    }

    pub fn name(&self) -> ir::Name { self.name.clone() }
    pub fn value(&self) -> ir::Value { *self.value.clone() }
    pub fn ty(&self) -> ir::Type {
        use ir::ValueTrait;
        self.value.ty()
    }

    /// Gets the ID of the global.
    ///
    /// The ID is guaranteed to be unique for each module.
    pub fn id(&self) -> util::Id { self.id }

    /// Sets the internal ID of the block.
    /// This **should not** be called directly.
    pub fn set_id(&mut self, id: util::Id) {
        self.id = id;
    }
}

impl std::fmt::Display for Global
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.name.fmt(fmt)
    }
}

