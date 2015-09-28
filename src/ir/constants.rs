
use ir::{types, Value, ValueTrait, Type, TypeTrait};
use util::{self,Upcast};
use bit_vec::BitVec;
use std::fmt;

use num::BigInt;
use num::bigint::ToBigInt;

pub trait ConstantTrait : Upcast<Constant> + ValueTrait
{
}

#[derive(Clone,Debug)]
pub enum Constant
{
    Integer(Integer),
    Float(Float),
    Struct(Struct),
}

impl Constant
{
    /// Creates an integer, returning `None` if `val` cannot fit into `ty`.
    pub fn integer<T: ToBigInt>(ty: types::Integer, val: T) -> Option<Constant> {
        Integer::new(ty,val).map(|a| a.upcast())
    }

    pub fn float(ty: types::Float, bits: BitVec) -> Constant {
        Float::new(ty,bits).upcast()
    }

    pub fn strukt(fields: Vec<Value>) -> Constant {
        Struct::new(fields).upcast()
    }

    pub fn unit_struct() -> Constant {
        Constant::strukt(Vec::new())
    }

    pub fn as_integer(&self) -> Option<&Integer> {
        if let &Constant::Integer(ref i) = self {
            Some(i)
        } else {
            None
        }
    }

    pub fn as_float(&self) -> Option<&Float> {
        if let &Constant::Float(ref i) = self {
            Some(i)
        } else {
            None
        }
    }

    pub fn as_struct(&self) -> Option<&Struct> {
        if let &Constant::Struct(ref i) = self {
            Some(i)
        } else {
            None
        }
    }
}

impl ValueTrait for Constant
{
    fn ty(&self) -> Type {
        match self {
            &Constant::Integer(ref val) => val.ty(),
            &Constant::Float(ref val) => val.ty(),
            &Constant::Struct(ref val) => val.ty(),
        }
    }
}

impl fmt::Display for Constant
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        match self {
            &Constant::Integer(ref val) => val.fmt(fmt),
            &Constant::Float(ref val) => val.fmt(fmt),
            &Constant::Struct(ref val) => val.fmt(fmt),
        }
    }
}

/// A constant integral value.
#[derive(Clone,Debug)]
pub struct Integer
{
    pub ty: types::Integer,
    pub value: BigInt,
}

impl Integer
{
    /// Creates a new constant integer, returning `None` if `val` cannot fit into `ty`.
    pub fn new<T: ToBigInt>(ty: types::Integer, val: T) -> Option<Self> {
        let bigint = val.to_bigint().expect("value cannot be converted into a integer");

        Integer::from_bigint(ty, bigint)
    }

    /// Creates a new constant integer from a `BigInt`, returning `None` if
    /// `val` cannot fit into `ty`.
    pub fn from_bigint(ty: types::Integer, val: BigInt) -> Option<Self> {
        let result = Integer {
            ty: ty,
            value: val,
        };

        // check if the value fits in `ty`.
        if result.fits_in_type(ty) {
            Some(result)
        } else { // the value cannot fit in `ty`
            None
        }
    }

    /// Creates a constant integer from an array of bytes representing an integer.
    /// Returns `None` if the integer cannot fit into `ty`.
    pub fn from_bytes_le(ty: types::Integer, sign: util::Sign, bytes: &[u8]) -> Option<Self> {
        Integer::from_bigint(ty, BigInt::from_bytes_le(sign.to_bigint_sign(), bytes))
    }

    /// Counts the number of bits that the magnitude takes up.
    pub fn count_magnitude_bits(&self) -> u64 {

        let (_,bytes) = self.value.to_bytes_le();
        
        // count all the bits in all but the most significant byte.
        let mut magnitude_size: u64 = (bytes.len() as u64 - 1)*8;
        magnitude_size += 8 - bytes.last().unwrap().leading_zeros() as u64;

        magnitude_size
    }

    /// Checks whether the value can fit in a given integer type.
    /// Note that if `ty` is unsigned, the function will check
    /// whether the value can fit in `ty` if it were unsigned.
    pub fn fits_in_type(&self, ty: types::Integer) -> bool {
        // if we need a sign bit, we have to take it
        // into account.
        let sign_bit_size = if ty.is_signed() { 1 } else { 0 };

        let magnitude_max_size: u64 = (ty.width() as u64) - sign_bit_size;

        self.count_magnitude_bits() <= magnitude_max_size
    }
}

impl ConstantTrait for Integer { }

impl Upcast<Value> for Integer
{
    fn upcast(self) -> Value {
        Value::Constant(self.upcast())
    }
}

impl ValueTrait for Integer
{
    fn ty(&self) -> Type { self.ty.clone().upcast() }
}

impl fmt::Display for Integer
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        self.value.fmt(fmt)
     }
}

/// A constant floating point value.
#[derive(Clone,Debug)]
pub struct Float
{
    ty: types::Float,
    bits: BitVec,
}

impl Float
{
    pub fn new(ty: types::Float, bits: BitVec) -> Self {
        Float {
            ty: ty,
            bits: bits,
        }
    }
}

impl ConstantTrait for Float { }

impl ValueTrait for Float
{
    fn ty(&self) -> Type { self.ty.clone().upcast() }
}

impl Upcast<Value> for Float
{
    fn upcast(self) -> Value {
        Value::Constant(self.upcast())
    }
}

impl fmt::Display for Float
{
    fn fmt(&self, _: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        unimplemented!()
    }
}

#[derive(Clone,Debug)]
pub struct Struct
{
    fields: Vec<Value>,
}

impl Struct
{
    pub fn new(fields: Vec<Value>) -> Self {

        Struct {
            fields: fields,
        }
    }
}

impl ConstantTrait for Struct { }

impl ValueTrait for Struct
{
    fn ty(&self) -> Type {
        // Create the struct type from the types of the values.
        types::Struct::new(
            self.fields.iter().map(|ref f| f.ty()).collect()
        ).upcast()
    }
}

impl Upcast<Value> for Struct
{
    fn upcast(self) -> Value {
        Value::Constant(self.upcast())
    }
}

impl fmt::Display for Struct
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        try!("{ ".fmt(fmt));

        try!(util::fmt_comma_separated_values(self.fields.iter(), fmt));

        " }".fmt(fmt)
    }
}

impl_upcast!(Constant,Value);
impl_upcast!(Integer,Constant);
impl_upcast!(Float,Constant);
impl_upcast!(Struct,Constant);

/// Tests that `Integer` can count its bits correctly.
#[test]
fn test_constantinteger_bitcount() {
    let types       = [ types::Integer::u64(), types::Integer::i64() ];
    let small_types = [ types::Integer::u(13), types::Integer::i(13) ];

    for (&ty,&small_ty) in types.iter().zip(small_types.iter()) {
        let i1 = Integer::new(ty, 255     ).unwrap();
        let i2 = Integer::new(ty, 127     ).unwrap();
        let i3 = Integer::new(ty, 1u64<<54).unwrap();

        assert_eq!(i1.count_magnitude_bits(), 8);
        assert_eq!(i2.count_magnitude_bits(), 7);
        assert_eq!(i3.count_magnitude_bits(), 55);

        assert!( i1.fits_in_type(small_ty));
        assert!( i2.fits_in_type(small_ty));
        assert!(!i3.fits_in_type(small_ty));
    }
}
