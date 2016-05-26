use {value,types,Value,Expression,ExpressionTrait,Type};
use std;

#[derive(Clone,Debug,PartialEq,Eq)]
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

    pub fn fields(&self) -> std::slice::Iter<Value> {
        self.fields.iter()
    }

    pub fn ty(&self) -> Type {
        // Create the struct type from the types of the values.
        types::Struct::new(
            self.fields.iter().map(|ref f| f.node.ty())
        ).into()
    }
}

impl value::LiteralTrait for Struct { }

impl ExpressionTrait for Struct { }

impl Into<Expression> for Struct
{
    fn into(self) -> Expression {
        Expression::Literal(self.into())
    }
}

impl Into<value::Literal> for Struct {
    fn into(self) -> value::Literal {
        value::Literal::Struct(self)
    }
}

