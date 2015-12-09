use {Instruction,Value,Expression,Type};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Div
{
    lhs: Box<Value>,
    rhs: Box<Value>,
}

impl Div
{
    pub fn new(lhs: Value, rhs: Value) -> Self {
        use lang::Value;
        assert!(lhs.ty() == rhs.ty());

        Div {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn ty(&self) -> Type { self.lhs.ty() }
}

impl_instruction!(Div: lhs, rhs);

