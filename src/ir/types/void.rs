use ir::types;
use std::fmt;

/// An empty type.
#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct Void;

impl Void
{
    pub fn void() -> Void { Void }
}

impl types::TypeTrait for Void
{
    fn size(&self) -> u64 { 0 }
}

impl fmt::Display for Void
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        "void".fmt(fmt)
    }
}

impl_type!(Void);
