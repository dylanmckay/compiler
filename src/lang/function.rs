
use lang;
use util;
use std;

use lang::Block;

#[derive(Clone,Debug)]
pub struct Signature<V: lang::Value>
{
    param_types: Vec<V::Type>,
    return_types: Vec<V::Type>,
}

impl<V> Signature<V>
    where V: lang::Value
{
    pub fn new() -> Self {
        Signature {
            param_types: Vec::new(),
            return_types: Vec::new(),
        }
    }

    pub fn ret(mut self, ty: V::Type) -> Self {
        self.return_types.push(ty);
        self
    }

    pub fn param(mut self, ty: V::Type) -> Self {
        self.param_types.push(ty);
        self
    }

    pub fn returns(&self) -> std::slice::Iter<V::Type> {
        self.return_types.iter()
    }

    pub fn parameters(&self) -> std::slice::Iter<V::Type> {
        self.param_types.iter()
    }
}

impl<V: lang::Value> std::cmp::PartialEq for Signature<V>
    where V::Type: std::cmp::PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.param_types == other.param_types &&
        self.return_types == other.return_types
    }
}

impl<V: lang::Value> std::cmp::Eq for Signature<V>
    where V::Type: std::cmp::Eq
{
}

#[derive(Clone,Debug)]
pub struct Function<V: lang::Value>
{
    id: util::Id,

    pub name: String,
    pub signature: Signature<V>,
    pub blocks: Vec<Block<V>>,
}

impl<V> Function<V>
    where V: lang::Value
{
    pub fn new(name: String,
               signature: Signature<V>,
               blocks: Vec<Block<V>>) -> Self {
        Function {
            id: util::Id::next(),

            name: name,
            signature: signature,
            blocks: blocks,
        }
    }

    pub fn empty(name: String, sig: Signature<V>) -> Self {
        Function::new(name, sig, Vec::new())
    }

    pub fn add(mut self, mut block: Block<V>) -> Self {
        // assign an ID to the block.
        block.set_id(util::Id::next());
        self.blocks.push(block);
        self
    }

    pub fn name(&self) -> &str { &self.name }

    pub fn signature(&self) -> &Signature<V> {
        &self.signature
    }

    pub fn flatten(self) -> Self {
        self.map_blocks(|b| b.flatten())
    }

    /// Gets the ID of the function.
    ///
    /// The ID is guaranteed to be unique for each module.
    pub fn id(&self) -> util::Id { self.id }

    pub fn blocks(&self) -> std::slice::Iter<Block<V>> {
        self.blocks.iter()
    }

    pub fn blocks_mut(&mut self) -> std::slice::IterMut<Block<V>> {
        self.blocks.iter_mut()
    }

    pub fn map_blocks<F>(mut self, f: F) -> Self
        where F: FnMut(Block<V>) -> Block<V> {

        let blocks = self.blocks.into_iter().map(f);
        self.blocks = blocks.collect();

        self
    }

    pub fn with_blocks<I>(mut self, blocks: I) -> Self
        where I: Iterator<Item=Block<V>> {

        self.blocks = blocks.collect();
        self
    }
}

impl<V: lang::Value> util::Identifiable for Function<V>
{
    fn get_id(&self) -> util::Id { self.id }
}

impl<V: PartialEq + lang::Value> std::cmp::PartialEq for Function<V>
{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.signature == other.signature
    }
}

impl<V: Eq + lang::Value> std::cmp::Eq for Function<V> { }
