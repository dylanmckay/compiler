use {value,types,Expression,ExpressionTrait,Type};
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

impl value::LiteralTrait for Decimal { }

impl ExpressionTrait for Decimal { }

impl Into<Expression> for Decimal
{
    fn into(self) -> Expression {
        Expression::Literal(self.into())
    }
}

impl Into<value::Literal> for Decimal {
    fn into(self) -> value::Literal {
        value::Literal::Decimal(self)
    }
}

