
use ir::{self,Instruction,Expression};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Shr
{
    value: Box<ir::Expression>,
    amount: Box<ir::Expression>,
}

impl Shr
{
    pub fn new(value: ir::Expression, amount: ir::Expression) -> Self {
        assert!(value.ty() == amount.ty());

        Shr {
            value: Box::new(value),
            amount: Box::new(amount),
        }
    }

    pub fn ty(&self) -> ir::Type { self.value.ty() }
}

impl_instruction!(Shr: value, amount);

