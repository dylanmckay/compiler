use {Instruction,Value,Expression,Type};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Add
{
    lhs: Box<Value>,
    rhs: Box<Value>,
}

impl Add
{
    pub fn new(lhs: Value, rhs: Value) -> Self {
        assert!(lhs.ty() == rhs.ty());

        Add {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn ty(&self) -> Type {
        self.lhs.ty()
    }
}

impl_instruction!(Add: lhs, rhs);
impl_instruction_binary!(Add: lhs, rhs);
