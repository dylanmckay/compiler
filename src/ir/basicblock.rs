
use ir::{self,Value};
use std::fmt;
use lang;

/// A basic block is a list of instructions which
/// end with a single terminator instruction.
#[derive(Clone,Debug)]
pub struct BasicBlock
{
    pub name: ir::Name,
    pub body: Vec<ir::Value>,
}

impl BasicBlock
{
    pub fn new(name: ir::Name, body: Vec<ir::Value>) -> BasicBlock {
        BasicBlock {
            name: name,
            body: body,
        }
    }

    pub fn empty(name: ir::Name) -> BasicBlock {
        BasicBlock::new(name, Vec::new())
    }

    pub fn add<T>(mut self, value: T) -> Self
        where T: Into<ir::Value> {
        self.body.push(value.into());
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

impl Into<Value> for BasicBlock
{
    fn into(self) -> Value { Value::BasicBlock(self) }
}
