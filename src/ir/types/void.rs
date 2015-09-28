use ir::types::{Type,TypeTrait};
use std::fmt;

/// An empty type.
#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct Void;

impl Void
{
    pub fn void() -> Void { Void }
}

impl TypeTrait for Void
{
    fn size(&self) -> u64 { 0 }
    fn upcast(self) -> Type { Type::Void(self) }
}

impl fmt::Display for Void
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        "void".fmt(fmt)
    }
}

impl_type!(Void);
