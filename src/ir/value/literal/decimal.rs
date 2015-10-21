
use ir::{self,types,Value,ValueTrait,Type};
use bit_vec::BitVec;


/// A decimal value.
#[derive(Clone,Debug,PartialEq,Eq)]
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

    pub fn ty(&self) -> Type { self.ty.clone().into() }
}

impl ir::value::LiteralTrait for Decimal { }

impl ValueTrait for Decimal { }

impl Into<Value> for Decimal
{
    fn into(self) -> Value {
        Value::Literal(self.into())
    }
}

impl Into<ir::value::Literal> for Decimal {
    fn into(self) -> ir::value::Literal {
        ir::value::Literal::Decimal(self)
    }
}

