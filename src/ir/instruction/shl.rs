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
        assert!(value.ty() == amount.ty());

        Shl {
            value: Box::new(value),
            amount: Box::new(amount),
        }
    }

    pub fn ty(&self) -> Type { self.value.ty() }
}

impl_instruction!(Shl: value, amount);

