
use lang;
use util;

/// A basic block is a list of instructions which
/// end with a single terminator instruction.
#[derive(Clone,Debug)]
pub struct Block<V>
{
    id: util::Id,

    pub name: lang::Name,
    pub body: Vec<V>,
}

impl<V> Block<V>
    where V: lang::Value
{
    pub fn new(name: lang::Name,
               body: Vec<V>) -> Self {
        Block {
            id: util::Id::next(),
            name: name,
            body: body,
        }
    }

    pub fn empty(name: lang::Name) -> Self {
        Block::new(name, Vec::new())
    }

    pub fn add<T>(&mut self, value: T)
        where T: Into<V> {
        self.body.push(value.into());
    }

    pub fn name(&self) -> &lang::Name { &self.name }

    pub fn flatten(self) -> Self {
        let mut block = Block {
            id: self.id.clone(),
            name: self.name.clone(),
            body: Vec::new(),
        };

        for value in self.subvalues() {
            let new_value = value.flatten(&mut block);
            block.add(new_value);
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

    pub fn map_subvalues<F>(mut self, mut f: F) -> Self
        where F: FnMut(V) -> V {
        self.body = self.body.into_iter().map(|a| f(a)).collect();
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

