use ir::types;
use std::fmt;

/// An empty type.
#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct Label;

impl Label
{
    pub fn new() -> Label { Label }
}

impl types::TypeTrait for Label
{
    fn size(&self) -> u64 { 0 }
}

impl fmt::Display for Label
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        "label".fmt(fmt)
    }
}

impl_type!(Label);
