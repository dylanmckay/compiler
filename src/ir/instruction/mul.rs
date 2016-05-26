use {Instruction,Value,Expression,Type};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Mul
{
    pub lhs: Box<Value>,
    pub rhs: Box<Value>,
}

impl Mul
{
    pub fn new(lhs: Value, rhs: Value) -> Self {
        Mul {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn ty(&self) -> Type { self.lhs.node.ty() }
}

impl_instruction!(Mul: lhs, rhs);
impl_instruction_binary!(Mul: lhs, rhs);
