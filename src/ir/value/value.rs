use {Expression,Spanned};

pub type Value = Spanned<Expression>;

impl Value
{
    // FIXME: temporary function
    pub fn new(expression: Expression) -> Self {
        Value {
            node: expression,
        }
    }
}

impl Into<Expression> for Value
{
    fn into(self) -> Expression {
        self.node
    }
}

impl Into<Value> for Expression
{
    fn into(self) -> Value {
        Value {
            node: self,
        }
    }
}

