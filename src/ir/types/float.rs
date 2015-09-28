
use ir::types::{Type,TypeTrait};
use std::fmt;

/// A floating point type.
#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct Float
{
    bit_width: u16,
}

impl Float
{
    pub fn new(bit_width: u16) -> Float {
        Float {
            bit_width: bit_width,
        }
    }

    pub fn f16() -> Float { Float::new(16) }
    pub fn f32() -> Float { Float::new(32) }
    pub fn f64() -> Float { Float::new(64) }
}

impl TypeTrait for Float
{
    fn size(&self) -> u64 {
        self.bit_width as u64
    }

    fn upcast(self) -> Type {
        Type::Float(self)
    }
}

impl fmt::Display for Float
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        try!('f'.fmt(fmt));
        self.bit_width.fmt(fmt)
    }
}

impl_type!(Float);
