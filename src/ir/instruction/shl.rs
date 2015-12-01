use ir::{self,Instruction,Value,Expression};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Shl
{
    value: Box<ir::Value>,
    amount: Box<ir::Value>,
}

impl Shl
{
    pub fn new(value: ir::Value, amount: ir::Value) -> Self {
        assert!(value.ty() == amount.ty());

        Shl {
            value: Box::new(value),
            amount: Box::new(amount),
        }
    }

    pub fn ty(&self) -> ir::Type { self.value.ty() }
}

impl_instruction!(Shl: value, amount);

