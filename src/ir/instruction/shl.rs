use {Instruction,Value,Expression,Type};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Shl
{
    value: Box<Value>,
    amount: Box<Value>,
}

impl Shl
{
    pub fn new(value: Value, amount: Value) -> Self {
        assert!(value.node.ty() == amount.node.ty());

        Shl {
            value: Box::new(value),
            amount: Box::new(amount),
        }
    }

    pub fn ty(&self) -> Type { self.value.node.ty() }
}

impl_instruction!(Shl: value, amount);
impl_instruction_binary!(Shl: value, amount);
