
use ir::{self,Instruction,Value,Expression};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Shr
{
    value: Box<ir::Value>,
    amount: Box<ir::Value>,
}

impl Shr
{
    pub fn new(value: ir::Value, amount: ir::Value) -> Self {
        assert!(value.ty() == amount.ty());

        Shr {
            value: Box::new(value),
            amount: Box::new(amount),
        }
    }

    pub fn ty(&self) -> ir::Type { self.value.ty() }
}

impl_instruction!(Shr: value, amount);

