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
        assert!(lhs.ty() == rhs.ty());

        Div {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn ty(&self) -> Type { self.lhs.ty() }
}

impl_instruction!(Div: lhs, rhs);
impl_instruction_binary!(Div: lhs, rhs);
