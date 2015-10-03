
use ir::types;
use std::fmt;

/// A decimal type.
#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct Decimal
{
    bit_width: u16,
}

impl Decimal
{
    pub fn new(bit_width: u16) -> Decimal {
        Decimal {
            bit_width: bit_width,
        }
    }

    pub fn f16() -> Decimal { Decimal::new(16) }
    pub fn f32() -> Decimal { Decimal::new(32) }
    pub fn f64() -> Decimal { Decimal::new(64) }
}

impl types::TypeTrait for Decimal
{
    fn size(&self) -> u64 {
        self.bit_width as u64
    }
}

impl fmt::Display for Decimal
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        try!('f'.fmt(fmt));
        self.bit_width.fmt(fmt)
    }
}

impl_type!(Decimal);
