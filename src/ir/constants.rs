
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
    Integer(ConstantInteger),
    Float(ConstantFloat),
    Struct(ConstantStruct),
}

impl Constant
{
    /// Creates an integer, returning `None` if `val` cannot fit into `ty`.
    pub fn integer<T: ToBigInt>(ty: types::Integer, val: T) -> Option<Constant> {
        ConstantInteger::new(ty,val).map(|a| a.upcast())
    }

    pub fn float(ty: types::Float, bits: BitVec) -> Constant {
        ConstantFloat::new(ty,bits).upcast()
    }

    pub fn strukt(fields: Vec<Value>) -> Constant {
        ConstantStruct::new(fields).upcast()
    }

    pub fn unit_struct() -> Constant {
        Constant::strukt(Vec::new())
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
pub struct ConstantInteger
{
    ty: types::Integer,
    value: BigInt,
}

impl ConstantInteger
{
    /// Creates a new constant integer, returning `None` if `val` cannot fit into `ty`.
    pub fn new<T: ToBigInt>(ty: types::Integer, val: T) -> Option<ConstantInteger> {
        let bigint = val.to_bigint().expect("value cannot be converted into a integer");

        ConstantInteger::from_bigint(ty, bigint)
    }

    /// Creates a new constant integer from a `BigInt`, returning `None` if
    /// `val` cannot fit into `ty`.
    pub fn from_bigint(ty: types::Integer, val: BigInt) -> Option<ConstantInteger> {
        let result = ConstantInteger {
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
    pub fn from_bytes_le(ty: types::Integer, sign: util::Sign, bytes: &[u8]) -> Option<ConstantInteger> {
        ConstantInteger::from_bigint(ty, BigInt::from_bytes_le(sign.to_bigint_sign(), bytes))
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

impl ConstantTrait for ConstantInteger { }

impl Upcast<Value> for ConstantInteger
{
    fn upcast(self) -> Value {
        Value::Constant(self.upcast())
    }
}

impl ValueTrait for ConstantInteger
{
    fn ty(&self) -> Type { self.ty.clone().upcast() }
}

impl fmt::Display for ConstantInteger
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        self.value.fmt(fmt)
     }
}

/// A constant floating point value.
#[derive(Clone,Debug)]
pub struct ConstantFloat
{
    ty: types::Float,
    bits: BitVec,
}

impl ConstantFloat
{
    pub fn new(ty: types::Float, bits: BitVec) -> Self {
        ConstantFloat {
            ty: ty,
            bits: bits,
        }
    }
}

impl ConstantTrait for ConstantFloat { }

impl ValueTrait for ConstantFloat
{
    fn ty(&self) -> Type { self.ty.clone().upcast() }
}

impl Upcast<Value> for ConstantFloat
{
    fn upcast(self) -> Value {
        Value::Constant(self.upcast())
    }
}

impl fmt::Display for ConstantFloat
{
    fn fmt(&self, _: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        unimplemented!()
    }
}

#[derive(Clone,Debug)]
pub struct ConstantStruct
{
    fields: Vec<Value>,
}

impl ConstantStruct
{
    pub fn new(fields: Vec<Value>) -> ConstantStruct {

        ConstantStruct {
            fields: fields,
        }
    }
}

impl ConstantTrait for ConstantStruct { }

impl ValueTrait for ConstantStruct
{
    fn ty(&self) -> Type {
        // Create the struct type from the types of the values.
        types::Struct::new(
            self.fields.iter().map(|ref f| f.ty()).collect()
        ).upcast()
    }
}

impl Upcast<Value> for ConstantStruct
{
    fn upcast(self) -> Value {
        Value::Constant(self.upcast())
    }
}

impl fmt::Display for ConstantStruct
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        try!("{ ".fmt(fmt));

        try!(util::fmt_comma_separated_values(self.fields.iter(), fmt));

        " }".fmt(fmt)
    }
}

impl_upcast!(Constant,Value);
impl_upcast!(ConstantInteger,Constant);
impl_upcast!(ConstantFloat,Constant);
impl_upcast!(ConstantStruct,Constant);

/// Tests that `ConstantInteger` can count its bits correctly.
#[test]
fn test_constantinteger_bitcount() {
    let types       = [ types::Integer::u64(), types::Integer::i64() ];
    let small_types = [ types::Integer::u(13), types::Integer::i(13) ];

    for (&ty,&small_ty) in types.iter().zip(small_types.iter()) {
        let i1 = ConstantInteger::new(ty, 255     ).unwrap();
        let i2 = ConstantInteger::new(ty, 127     ).unwrap();
        let i3 = ConstantInteger::new(ty, 1u64<<54).unwrap();

        assert_eq!(i1.count_magnitude_bits(), 8);
        assert_eq!(i2.count_magnitude_bits(), 7);
        assert_eq!(i3.count_magnitude_bits(), 55);

        assert!( i1.fits_in_type(small_ty));
        assert!( i2.fits_in_type(small_ty));
        assert!(!i3.fits_in_type(small_ty));
    }
}
