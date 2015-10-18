
use ir::{self,Value};
use lang;
use util;

/// A basic block is a list of instructions which
/// end with a single terminator instruction.
#[derive(Clone,Debug)]
pub struct Block
{
    id: util::Id,

    pub name: ir::Name,
    pub body: Vec<ir::Value>,
}

impl Block
{
    pub fn new(name: ir::Name,
               body: Vec<ir::Value>) -> Block {
        Block {
            id: util::Id::unspecified(),
            name: name,
            body: body,
        }
    }

    pub fn empty(name: ir::Name) -> Block {
        Block::new(name, Vec::new())
    }

    pub fn add<T>(mut self, value: T) -> Self
        where T: Into<ir::Value> {
        self.body.push(value.into());
        self
    }

    pub fn name(&self) -> &ir::Name { &self.name }

    pub fn reference(&self) -> ir::Value {
        ir::Value::block_ref(self)
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
}

impl lang::Block for Block
{
    type Value = ir::Value;

    fn subvalues(&self) -> Vec<ir::Value> {
        self.body.clone()
    }

    fn with_subvalues<I>(mut self, values: I) -> Self
        where I: Iterator<Item=ir::Value> {

        self.body = values.collect();
        self
    }

    fn map_subvalues<F>(mut self, mut f: F) -> Self
        where F: FnMut(ir::Value) -> ir::Value {
        self.body = self.body.into_iter().map(|a| f(a)).collect();
        self
    }
}

