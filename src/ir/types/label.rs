use ir::types::{Type,TypeTrait};
use std::fmt;

/// An empty type.
#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct Label;

impl Label
{
    pub fn new() -> Label { Label }
}

impl TypeTrait for Label
{
    fn size(&self) -> u64 { 0 }
    fn upcast(self) -> Type { Type::Label(self) }
}

impl fmt::Display for Label
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        "label".fmt(fmt)
    }
}

impl_type!(Label);
