use {Instruction,Value,Expression,Type};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Sub
{
    lhs: Box<Value>,
    rhs: Box<Value>,
}

impl Sub
{
    pub fn new(lhs: Value, rhs: Value) -> Self {
        assert!(lhs.node.ty() == rhs.node.ty());

        Sub {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn ty(&self) -> Type { self.lhs.node.ty() }
}

impl_instruction!(Sub: lhs, rhs);
impl_instruction_binary!(Sub: lhs, rhs);
