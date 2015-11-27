
use ir::{self,Instruction,Expression};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Shl
{
    value: Box<ir::Expression>,
    amount: Box<ir::Expression>,
}

impl Shl
{
    pub fn new(value: ir::Expression, amount: ir::Expression) -> Self {
        assert!(value.ty() == amount.ty());

        Shl {
            value: Box::new(value),
            amount: Box::new(amount),
        }
    }

    pub fn ty(&self) -> ir::Type { self.value.ty() }
}

impl_instruction!(Shl: value, amount);

