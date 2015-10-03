
use ir::{self,types,Value,ValueTrait,Type};
use bit_vec::BitVec;
use std::fmt;


/// A decimal value.
#[derive(Clone,Debug)]
pub struct Decimal
{
    ty: types::Decimal,
    bits: BitVec,
}

impl Decimal
{
    pub fn new(ty: types::Decimal, bits: BitVec) -> Self {
        Decimal {
            ty: ty,
            bits: bits,
        }
    }
}

impl ir::value::ConstantTrait for Decimal { }

impl ValueTrait for Decimal
{
    fn ty(&self) -> Type { self.ty.clone().into() }
}

impl Into<Value> for Decimal
{
    fn into(self) -> Value {
        Value::Constant(self.into())
    }
}

impl fmt::Display for Decimal
{
    fn fmt(&self, _: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        unimplemented!()
    }
}

impl Into<ir::value::Constant> for Decimal {
    fn into(self) -> ir::value::Constant {
        ir::value::Constant::Decimal(self)
    }
}

