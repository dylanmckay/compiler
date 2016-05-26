use {Instruction,Value,Expression,Type};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Shr
{
    value: Box<Value>,
    amount: Box<Value>,
}

impl Shr
{
    pub fn new(value: Value, amount: Value) -> Self {
        assert!(value.node.ty() == amount.node.ty());

        Shr {
            value: Box::new(value),
            amount: Box::new(amount),
        }
    }

    pub fn ty(&self) -> Type { self.value.node.ty() }
}

impl_instruction!(Shr: value, amount);
impl_instruction_binary!(Shr: value, amount);
