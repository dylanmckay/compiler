

use ir::{self,types,Value,ValueTrait,Type};
use util;
use std;

use num::BigInt;
use num::bigint::ToBigInt;

/// A constant integral value.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Integer
{
    ty: types::Integer,
    value: BigInt,
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

    pub fn integer_ty(&self) -> types::Integer { self.ty.clone() }
    pub fn ty(&self) -> Type { self.ty.clone().into() }
    pub fn value(&self) -> BigInt { self.value.clone() }
}

impl std::ops::Add for Integer
{
    type Output = Integer;

    fn add(mut self, rhs: Integer) -> Integer {
        let val = self.value + rhs.value;
        self.value = val;
        self
    }
}

impl std::ops::Sub for Integer
{
    type Output = Integer;

    fn sub(mut self, rhs: Integer) -> Integer {
        let val = self.value - rhs.value;
        self.value = val;
        self
    }
}

impl std::ops::Mul for Integer
{
    type Output = Integer;

    fn mul(mut self, rhs: Integer) -> Integer {
        let val = self.value * rhs.value;
        self.value = val;
        self
    }
}

impl std::ops::Div for Integer
{
    type Output = Integer;

    fn div(mut self, rhs: Integer) -> Integer {
        // FIXME: this will break with division by zero
        let val = self.value / rhs.value;
        self.value = val;
        self
    }
}

impl std::ops::Shl<Integer> for Integer
{
    type Output = Integer;

    fn shl(mut self, rhs: Integer) -> Integer {
        use num::traits::ToPrimitive;

        let val = self.value << rhs.value.to_usize().unwrap();
        self.value = val;
        self
    }
}

impl std::ops::Shr<Integer> for Integer
{
    type Output = Integer;

    fn shr(mut self, rhs: Integer) -> Integer {
        use num::traits::ToPrimitive;

        let val = self.value >> rhs.value.to_usize().unwrap();
        self.value = val;
        self
    }
}

impl ir::value::literal::LiteralTrait for Integer { }

impl ValueTrait for Integer { }

impl Into<Value> for Integer
{
    fn into(self) -> Value {
        Value::Literal(self.into())
    }
}

impl Into<ir::value::Literal> for Integer {
    fn into(self) -> ir::value::Literal {
        ir::value::Literal::Integer(self)
    }
}

