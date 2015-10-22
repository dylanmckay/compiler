
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
    pub fn new<N>(name: N,
                  body: Vec<V>) -> Self
        where N: Into<String> {
        Block {
            id: util::Id::next(),
            name: name.into(),
            body: body,
        }
    }

    pub fn empty<N>(name: N) -> Self
        where N: Into<String> {
        Block::new(name, Vec::new())
    }

    pub fn append_value<T>(&mut self, value: T)
        where T: Into<V> {
        self.body.push(value.into());
    }

    pub fn name(&self) -> &str { &self.name }

    pub fn flatten(self) -> Self {
        let mut block = Block {
            id: self.id.clone(),
            name: self.name.clone(),
            body: Vec::new(),
        };

        for value in self.subvalues() {
            let new_value = value.flatten(&mut block);
            block.append_value(new_value);
        }

        block
    }

    /// Gets the ID of the block.
    ///
    /// The ID is guaranteed to be unique for each function.
    pub fn id(&self) -> util::Id { self.id }

    /// Sets the internal ID of the block.
    /// This **should not** be called directly.
    pub fn set_id(&mut self, id: util::Id) {
        self.id = id;
    }

    pub fn subvalues(&self) -> Vec<V> {
        self.body.clone()
    }

    pub fn with_subvalues<I>(mut self, values: I) -> Self
        where I: Iterator<Item=V> {

        self.body = values.collect();
        self
    }

    pub fn map_subvalues<F>(mut self, f: F) -> Self
        where F: FnMut(V) -> V {
        self.body = self.body.into_iter().map(f).collect();
        self
    }

    pub fn filter<F>(mut self, mut f: F) -> Self
        where F: FnMut(&V) -> bool {
        self.body = self.body.into_iter()
                             .filter(|a| f(a))
                             .collect();
        self
    }
}

