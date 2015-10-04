
use ir::{self,types,Value,Name,Block};
use std::{self,fmt};
use lang;

#[derive(Clone,Debug)]
pub struct Function
{
    pub name: Name,
    pub signature: types::Function,
    pub blocks: Vec<Block>,
}

impl Function
{
    pub fn new(name: Name,
               signature: types::Function,
               blocks: Vec<Block>) -> Function {
        Function {
            name: name,
            signature: signature,
            blocks: blocks,
        }
    }

    pub fn empty(name: Name, ty: types::Function) -> Function {
        Function::new(name, ty, Vec::new())
    }

    pub fn add(mut self, block: Block) -> Function {
        self.blocks.push(block);
        self
    }

    pub fn name(&self) -> &Name { &self.name }

    pub fn signature(&self) -> &types::Function {
        &self.signature
    }
}

impl ir::ValueTrait for Function
{
    fn ty(&self) -> ir::Type {
        self.signature.clone().into()
    }
}

impl Into<Value> for Function
{
    fn into(self) -> Value {
        Value::Function(self)
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


