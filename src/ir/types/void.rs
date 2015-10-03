use ir::types;
use std::fmt;

/// An empty type.
#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct Void;

impl Void
{
    /// The `void` type.
    pub fn void() -> Void { Void }
}

impl fmt::Display for Void
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        "void".fmt(fmt)
    }
}

impl types::TypeTrait for Void { }

impl_type!(Void);
