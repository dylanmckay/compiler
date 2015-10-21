
use lang;
use util;

use std;

#[derive(Clone,Debug)]
pub struct Global<V: lang::Value>
{
    id: util::Id,

    name: String,
    // TODO: does this need to be boxed
    value: Box<V>,
}

impl<V> Global<V>
    where V: lang::Value
{
    pub fn new(name: String, value: V) -> Self {
        Global {
            id: util::Id::next(),

            name: name,
            value: Box::new(value),
        }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn value(&self) -> V { *self.value.clone() }
    pub fn ty(&self) -> V::Type {
        self.value.ty()
    }

    /// Gets the ID of the global.
    ///
    /// The ID is guaranteed to be unique for each module.
    pub fn id(&self) -> util::Id { self.id }
}

impl<V: lang::Value> util::Identifiable for Global<V>
{
    fn get_id(&self) -> util::Id { self.id }
}

impl<V: lang::Value> std::fmt::Display for Global<V>
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.name.fmt(fmt)
    }
}

