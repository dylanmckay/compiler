
use ir::{self,types,Block};
use std::{self,fmt};
use lang;
use util;

#[derive(Clone,Debug)]
pub struct Function
{
    id: util::Id,

    pub name: String,
    pub signature: types::Function,
    pub blocks: Vec<Block>,

    generator: util::id::Generator,
}

impl Function
{
    pub fn new(name: String,
               signature: types::Function,
               blocks: Vec<Block>) -> Function {
        Function {
            id: util::Id::unspecified(),

            name: name,
            signature: signature,
            blocks: blocks,

            generator: util::id::Generator::new(),
        }
    }

    pub fn empty(name: String, ty: types::Function) -> Function {
        Function::new(name, ty, Vec::new())
    }

    pub fn add(mut self, mut block: Block) -> Function {
        // assign an ID to the block.
        
        block.set_id(self.generator.next());
        self.blocks.push(block);
        self
    }

    pub fn name(&self) -> &str { &self.name }

    pub fn signature(&self) -> &types::Function {
        &self.signature
    }

    /// Gets the ID of the function.
    ///
    /// The ID is guaranteed to be unique for each module.
    pub fn id(&self) -> util::Id { self.id }
}

impl util::id::Identifiable for Function
{
    fn set_id(&mut self, id: util::Id) {
        self.id = id;
    }
}

impl fmt::Display for Function
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        ir::print::function(self, fmt)
    }
}

impl lang::Function for Function
{
    type Block = Block;
    type Type = ir::Type;

    fn blocks<'a>(&'a self) -> std::slice::Iter<'a,Block> {
        self.blocks.iter()
    }

    fn blocks_mut<'a>(&'a mut self) -> std::slice::IterMut<'a,Block> {
        self.blocks.iter_mut()
    }

    fn map_blocks<F>(mut self, mut f: F) -> Self
        where F: FnMut(Block) -> Block {

        let blocks = self.blocks.into_iter().map(|a| f(a));
        self.blocks = blocks.collect();

        self
    }

    fn with_blocks<I>(mut self, blocks: I) -> Self
        where I: Iterator<Item=Block> {

        self.blocks = blocks.collect();
        self
    }
}

impl std::cmp::PartialEq for Function
{
    fn eq(&self, other: &Function) -> bool {
        self.name == other.name &&
        self.signature == other.signature
    }
}

impl std::cmp::Eq for Function { }
