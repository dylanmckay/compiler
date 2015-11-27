
use ir::{self,types,Expression,ExpressionTrait,Type};
use std;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Struct
{
    fields: Vec<Expression>,
}

impl Struct
{
    pub fn new(fields: Vec<Expression>) -> Self {

        Struct {
            fields: fields,
        }
    }

    pub fn fields(&self) -> std::slice::Iter<Expression> {
        self.fields.iter()
    }

    pub fn ty(&self) -> Type {
        use lang::Value;

        // Create the struct type from the types of the values.
        types::Struct::new(
            self.fields.iter().map(|ref f| f.ty())
        ).into()
    }
}

impl ir::value::LiteralTrait for Struct { }

impl ExpressionTrait for Struct { }

impl Into<Expression> for Struct
{
    fn into(self) -> Expression {
        Expression::Literal(self.into())
    }
}

impl Into<ir::value::Literal> for Struct {
    fn into(self) -> ir::value::Literal {
        ir::value::Literal::Struct(self)
    }
}

