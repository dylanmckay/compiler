use Value;
use Type;
use util;

use std;

/// A global variable.
#[derive(Clone,Debug)]
pub struct Global
{
    id: util::Id,
    /// The name of the global.
    name: String,
    /// The variable value.
    value: Value,
}

impl Global
{
    /// Creates a new global variable.
    pub fn new(name: String, value: Value) -> Self {
        Global {
            id: util::Id::next(),

            name: name,
            value: value,
        }
    }

    /// Gets the name of the global.
    pub fn name(&self) -> &str { &self.name }
    /// Gets the value of the global.
    pub fn value(&self) -> &Value { &self.value }
    /// Gets the type of the value the global contains.
    pub fn ty(&self) -> Type { self.value.node.ty() }
    /// Gets the ID of the global.
    pub fn id(&self) -> util::Id { self.id }

    pub fn map_value<F>(mut self, mut f: F) -> Self
        where F: FnMut(Value) -> Value {
        self.value = f(self.value);
        self
    }
}

impl util::Identifiable for Global
{
    fn get_id(&self) -> util::Id { self.id }
    fn internal_set_id(&mut self, id: util::Id) {
        self.id = id;
    }
}

impl std::fmt::Display for Global
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.name.fmt(fmt)
    }
}

