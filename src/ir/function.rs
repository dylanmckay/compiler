
use ir::{self,types,Value,Block};
use std::{self,fmt};
use lang;

#[derive(Clone,Debug)]
pub struct Function
{
    // TODO: functions must have textual names. use String
    pub name: String,
    pub signature: types::Function,
    pub blocks: Vec<Block>,
}

impl Function
{
    pub fn new(name: String,
               signature: types::Function,
               blocks: Vec<Block>) -> Function {
        Function {
            name: name,
            signature: signature,
            blocks: blocks,
        }
    }

    pub fn empty(name: String, ty: types::Function) -> Function {
        Function::new(name, ty, Vec::new())
    }

    pub fn add(mut self, block: Block) -> Function {
        self.blocks.push(block);
        self
    }

    pub fn name(&self) -> &str { &self.name }

    pub fn signature(&self) -> &types::Function {
        &self.signature
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
