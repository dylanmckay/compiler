
use lang;
use util;

/// A basic block is a list of instructions which
/// end with a single terminator instruction.
#[derive(Clone,Debug)]
pub struct Block<V>
{
    id: util::Id,

    name: String,
    body: Vec<V>,
}

impl<V> Block<V>
    where V: lang::Value
{
    /// Creates a new basic block.
    pub fn new<N>(name: N,
                  body: Vec<V>) -> Self
        where N: Into<String> {
        Block {
            id: util::Id::next(),
            name: name.into(),
            body: body,
        }
    }

    /// Creates a basic block with no contained instructions.
    pub fn empty<N>(name: N) -> Self
        where N: Into<String> {
        Block::new(name, Vec::new())
    }

    /// Appends a value to the basic block.
    pub fn append_value<T>(&mut self, value: T)
        where T: Into<V> {
        self.body.push(value.into());
    }

    /// Gets the name of the block.
    pub fn name(&self) -> &str { &self.name }

    /// Flattens the values in the block.
    pub fn flatten(self) -> Self {
        let mut block = Block {
            id: self.id.clone(),
            name: self.name.clone(),
            body: Vec::new(),
        };

        for value in self.body {
            let new_value = value.flatten(&mut block);
            block.append_value(new_value);
        }

        block
    }

    /// Gets the ID of the block.
    pub fn id(&self) -> util::Id { self.id }

    /// Gets the values that the block contains.
    pub fn values(&self) -> ::std::slice::Iter<V> {
        self.body.iter()
    }

    /// Gets the values that the block contains as mutable.
    pub fn values_mut(&mut self) -> ::std::slice::IterMut<V> {
        self.body.iter_mut()
    }

    /// Sets the values that the block contains.
    pub fn with_values<I>(mut self, values: I) -> Self
        where I: Iterator<Item=V> {

        self.body = values.collect();
        self
    }

    /// Performs a mapping on the values that the block contains.
    pub fn map_values<F>(mut self, f: F) -> Self
        where F: FnMut(V) -> V {
        self.body = self.body.into_iter().map(f).collect();
        self
    }

    /// Filters values out of the basic block.
    pub fn filter<F>(mut self, mut f: F) -> Self
        where F: FnMut(&V) -> bool {
        self.body = self.body.into_iter()
                             .filter(|a| f(a))
                             .collect();
        self
    }
}

