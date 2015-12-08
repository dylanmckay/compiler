use Value;
use util;

use std;

/// A global variable.
#[derive(Clone,Debug)]
pub struct Global<V: Value>
{
    id: util::Id,
    /// The name of the global.
    name: String,
    /// The variable value.
    value: V,
}

impl<V> Global<V>
    where V: Value
{
    /// Creates a new global variable.
    pub fn new(name: String, value: V) -> Self {
        Global {
            id: util::Id::next(),

            name: name,
            value: value,
        }
    }

    /// Gets the name of the global.
    pub fn name(&self) -> &str { &self.name }
    /// Gets the value of the global.
    pub fn value(&self) -> &V { &self.value }
    /// Gets the type of the value the global contains.
    pub fn ty(&self) -> V::Type { self.value.ty() }
    /// Gets the ID of the global.
    pub fn id(&self) -> util::Id { self.id }
}

impl<V: Value> util::Identifiable for Global<V>
{
    fn get_id(&self) -> util::Id { self.id }
}

impl<V: Value> std::fmt::Display for Global<V>
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.name.fmt(fmt)
    }
}

