
use ir;
use std::fmt;

/// A pointer.
#[derive(Clone,Debug)]
pub struct Pointer
{
    to: Box<ir::Value>,
}

impl Pointer
{
    pub fn to(value: ir::Value) -> Self {
        Pointer {
            to: Box::new(value),
        }
    }
}

impl fmt::Display for Pointer
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "&{}", self.to)
    }
}
