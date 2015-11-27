
use ir;

/// A pointer.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Pointer
{
    to: Box<ir::Expression>,
}

impl Pointer
{
    pub fn to(value: ir::Expression) -> Self {
        Pointer {
            to: Box::new(value),
        }
    }

    pub fn deref(self) -> ir::Expression {
        *self.to
    }

    pub fn underlying(&self) -> &ir::Expression {
        &self.to
    }

    pub fn ty(&self) -> ir::Type {
        use lang::Value;
        ir::Type::pointer(self.to.ty())
    }
}

impl ir::value::ExpressionTrait for Pointer { }

impl Into<ir::Expression> for Pointer
{
    fn into(self) -> ir::Expression {
        ir::Expression::Pointer(self)
    }
}
