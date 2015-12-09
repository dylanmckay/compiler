use types;
use std::fmt;

/// A basic block type.
#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct Block;

impl Block
{
    /// Creates a new block type.
    pub fn new() -> Block { Block }
}

impl fmt::Display for Block
{
    fn fmt(&self, _: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        unreachable!("basic blocks types cannot be printed");
    }
}

impl types::TypeTrait for Block { }

impl_type!(Block);
