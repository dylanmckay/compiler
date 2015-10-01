
use ir::{self,types,Value,ValueTrait,Type};
use bit_vec::BitVec;
use std::fmt;


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

impl ir::constants::ConstantTrait for Float { }

impl ValueTrait for Float
{
    fn ty(&self) -> Type { self.ty.clone().into() }
}

impl Into<Value> for Float
{
    fn into(self) -> Value {
        Value::Constant(self.into())
    }
}

impl fmt::Display for Float
{
    fn fmt(&self, _: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        unimplemented!()
    }
}

impl Into<ir::Constant> for Float {
    fn into(self) -> ir::Constant {
        ir::Constant::Float(self)
    }
}

