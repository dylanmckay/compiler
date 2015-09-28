
use ir::{self,Value};
use std::{self,fmt};
use lang;

/// A basic block is a list of instructions which
/// end with a single terminator instruction.
#[derive(Clone,Debug)]
pub struct BasicBlock
{
    pub name: ir::Name,
    pub body: Vec<ir::Instruction>,
}

impl BasicBlock
{
    pub fn new(name: ir::Name, body: Vec<ir::Instruction>) -> BasicBlock {
        BasicBlock {
            name: name,
            body: body,
        }
    }

    pub fn empty(name: ir::Name) -> BasicBlock {
        BasicBlock::new(name, Vec::new())
    }

    pub fn add(mut self, instruction: ir::Instruction) -> Self {
        self.body.push(instruction);

        self
    }
}

impl ir::ValueTrait for BasicBlock
{
    fn ty(&self) -> ir::Type {
        ir::Type::label()
    }
}

impl fmt::Display for BasicBlock
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        try!(write!(fmt, "{}:\n", self.name));

        for inst in self.body.iter() {
            try!(write!(fmt, "\t{}\n", inst));
        }

        Ok(())
    }
}

impl lang::BasicBlock for BasicBlock
{
    type Instruction = ir::Instruction;

    fn instructions<'a>(&'a self) -> std::slice::Iter<'a, ir::Instruction> {
        self.body.iter()
    }

    fn instructions_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, ir::Instruction> {
        self.body.iter_mut()
    }

    fn map_instructions<F>(mut self, mut f: F) -> Self
        where F: FnMut(ir::Instruction) -> ir::Instruction {

        let instrs = self.body.into_iter().map(|a| f(a));
        self.body = instrs.collect();

        self
    }

    fn with_instructions<I>(mut self, instructions: I) -> Self
        where I: Iterator<Item=ir::Instruction> {

        self.body = instructions.collect();
        self
    }
}

impl_upcast!(BasicBlock,Value);
