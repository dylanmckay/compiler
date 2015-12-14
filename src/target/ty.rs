use lang;
use std;

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Type
{
    Integer(u16),
}

impl lang::Type for Type { }

impl std::fmt::Display for Type
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        unimplemented!();
    }
}
