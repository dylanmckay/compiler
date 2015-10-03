
use ir::types;
use util::IntegerKind;
use std::fmt;

/// An integer type.
#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct Integer
{
    kind: IntegerKind,
    bit_width: u16,
}

impl Integer
{
    pub fn new(kind: IntegerKind, bit_width: u16) -> Integer {
        Integer {
            kind: kind,
            bit_width: bit_width,
        }
    }

    /// Creates a signed integer type.
    pub fn signed(bit_width: u16) -> Integer {
        Integer::new(IntegerKind::Signed, bit_width)
    }

    /// Createss an unsigned integer type.
    pub fn unsigned(bit_width: u16) -> Integer {
         Integer::new(IntegerKind::Unsigned, bit_width)
    }

    pub fn i(bit_width: u16) -> Integer {
        Integer::signed(bit_width)
    }

    pub fn u(bit_width: u16) -> Integer {
        Integer::unsigned(bit_width)
    }

    pub fn i8()   -> Integer { Integer::i(8)  }
    pub fn i16()  -> Integer { Integer::i(16) }
    pub fn i32()  -> Integer { Integer::i(32) }
    pub fn i64()  -> Integer { Integer::i(64) }
    pub fn i128() -> Integer { Integer::i(128) }
    
    pub fn u8()   -> Integer { Integer::u(8)  }
    pub fn u16()  -> Integer { Integer::u(16) }
    pub fn u32()  -> Integer { Integer::u(32) }
    pub fn u64()  -> Integer { Integer::u(64) }
    pub fn u128() -> Integer { Integer::u(128) }

    pub fn is_signed(self) -> bool { self.kind == IntegerKind::Signed }
    pub fn width(self) -> u16 { self.bit_width }
}

impl fmt::Display for Integer
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error>
    {
        try!(self.kind.prefix().fmt(fmt));
        self.bit_width.fmt(fmt)
    }
}

impl types::TypeTrait for Integer
{
    fn size(&self) -> u64 { self.bit_width as u64 }
}

impl_type!(Integer);
