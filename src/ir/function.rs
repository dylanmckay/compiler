
use ir::{self,types,Value,Name,BasicBlock};
use std::{self,fmt};
use lang;
use util;

#[derive(Clone,Debug)]
pub struct Function
{
    pub name: Name,
    pub signature: types::Signature,
    pub basicblocks: Vec<BasicBlock>,
}

impl Function
{
    pub fn new(name: Name,
               signature: types::Signature,
               basicblocks: Vec<BasicBlock>) -> Function {
        Function {
            name: name,
            signature: signature,
            basicblocks: basicblocks,
        }
    }

    pub fn empty(name: Name, signature: types::Signature) -> Function {
        Function::new(name, signature, Vec::new())
    }

    pub fn add(mut self, basicblock: BasicBlock) -> Function {
        self.basicblocks.push(basicblock);
        self
    }
}

impl ir::ValueTrait for Function
{
    fn ty(&self) -> ir::Type {
        use ir::TypeTrait;
        self.signature.clone().upcast()
    }
}

impl fmt::Display for Function
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        try!("define ".fmt(fmt));

        try!(util::fmt_comma_separated_values(self.signature.return_types.iter(), fmt));
        
        try!(write!(fmt, " @{}(", self.name));

        try!(util::fmt_comma_separated_values(self.signature.param_types.iter(), fmt));

        try!(") {\n".fmt(fmt));

        for bb in self.basicblocks.iter() {
            try!(bb.fmt(fmt));
        }

        "}\n".fmt(fmt)
    }
}

impl lang::Function for Function
{
    type BasicBlock = BasicBlock;
    type Type = ir::Type;

    fn basic_blocks<'a>(&'a self) -> std::slice::Iter<'a,BasicBlock> {
        self.basicblocks.iter()
    }

    fn basic_blocks_mut<'a>(&'a mut self) -> std::slice::IterMut<'a,BasicBlock> {
        self.basicblocks.iter_mut()
    }
    fn signature<'a>(&'a self) -> &'a lang::Signature<ir::Type> {
        &self.signature
    }
}

impl_upcast!(Function,Value);
