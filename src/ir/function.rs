
use ir::{self,types,Value,Name,Block};
use std::{self,fmt};
use lang;
use util;

#[derive(Clone,Debug)]
pub struct Function
{
    pub name: Name,
    pub signature: types::Signature,
    pub basicblocks: Vec<Block>,
}

impl Function
{
    pub fn new(name: Name,
               signature: types::Signature,
               basicblocks: Vec<Block>) -> Function {
        Function {
            name: name,
            signature: signature,
            basicblocks: basicblocks,
        }
    }

    pub fn empty(name: Name, signature: types::Signature) -> Function {
        Function::new(name, signature, Vec::new())
    }

    pub fn add(mut self, basicblock: Block) -> Function {
        self.basicblocks.push(basicblock);
        self
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
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {

        let mut accum = 0;
        try!(write!(fmt, "define {} @{}({}) {{\n",
                         util::comma_separated_values(self.signature.return_types.iter()),
                         self.name,
                         util::comma_separated_values(self.signature.param_types.iter())));

        for bb in self.basicblocks.iter() {
            try!(write!(fmt, "{}:\n", bb.name));

            for value in bb.body.iter() {
                 try!(write!(fmt, "\t%{} = {}\n", accum, value));

                 accum += 1;
            }
        }

        "}\n".fmt(fmt)
    }
}

impl lang::Function for Function
{
    type Block = Block;
    type Type = ir::Type;

    fn blocks<'a>(&'a self) -> std::slice::Iter<'a,Block> {
        self.basicblocks.iter()
    }

    fn blocks_mut<'a>(&'a mut self) -> std::slice::IterMut<'a,Block> {
        self.basicblocks.iter_mut()
    }

    fn map_blocks<F>(mut self, mut f: F) -> Self
        where F: FnMut(Block) -> Block {

        let blocks = self.basicblocks.into_iter().map(|a| f(a));
        self.basicblocks = blocks.collect();

        self
    }

    fn with_blocks<I>(mut self, blocks: I) -> Self
        where I: Iterator<Item=Block> {

        self.basicblocks = blocks.collect();
        self
    }

    fn signature<'a>(&'a self) -> &'a lang::Signature<ir::Type> {
        &self.signature
    }
}

