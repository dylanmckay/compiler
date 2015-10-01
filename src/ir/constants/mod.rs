
pub use self::integer::Integer;
pub use self::decimal::Decimal;
pub use self::strukt::Struct;

use ir::{types,Value,ValueTrait,Type};
use bit_vec::BitVec;
use std::fmt;

use num::bigint::ToBigInt;

/// Integer constant implementation.
pub mod integer;
/// Decimal constant implementation.
pub mod decimal;
/// Structure-constant implementation.
pub mod strukt;

pub trait ConstantTrait : Into<Constant> + ValueTrait
{
}

#[derive(Clone,Debug)]
pub enum Constant
{
    Integer(Integer),
    Decimal(Decimal),
    Struct(Struct),
}

impl Constant
{
    /// Creates an integer, returning `None` if `val` cannot fit into `ty`.
    pub fn integer<T: ToBigInt>(ty: types::Integer, val: T) -> Option<Constant> {
        Integer::new(ty,val).map(|a| a.into())
    }

    pub fn decimal(ty: types::Decimal, bits: BitVec) -> Constant {
        Decimal::new(ty,bits).into()
    }

    pub fn strukt(fields: Vec<Value>) -> Constant {
        Struct::new(fields).into()
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

    pub fn as_decimal(&self) -> Option<&Decimal> {
        if let &Constant::Decimal(ref i) = self {
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
            &Constant::Decimal(ref val) => val.ty(),
            &Constant::Struct(ref val) => val.ty(),
        }
    }
}

impl fmt::Display for Constant
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        match self {
            &Constant::Integer(ref val) => val.fmt(fmt),
            &Constant::Decimal(ref val) => val.fmt(fmt),
            &Constant::Struct(ref val) => val.fmt(fmt),
        }
    }
}

impl Into<Value> for Constant
{
    fn into(self) -> Value {
        Value::Constant(self)
    }
}

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
