
use ir;
use std::fmt;

/// A pointer.
#[derive(Clone,Debug,Eq,PartialEq)]
pub struct Pointer
{
    to: Box<ir::Type>,
}

impl Pointer
{
    /// Creates a pointer. 
    pub fn to(value: ir::Type) -> Self {
        Pointer {
            to: Box::new(value),
        }
    }
}

impl fmt::Display for Pointer
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}*", self.to)
    }
}

impl ir::TypeTrait for Pointer
{
    fn size(&self) -> u64 {
        // we need to know more about the target
        unimplemented!();
    }
}


impl_type!(Pointer);
