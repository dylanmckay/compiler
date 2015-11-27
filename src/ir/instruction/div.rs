
use ir::{self,Instruction,Expression};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Div
{
    lhs: Box<ir::Expression>,
    rhs: Box<ir::Expression>,
}

impl Div
{
    pub fn new(lhs: ir::Expression, rhs: ir::Expression) -> Self {
        use lang::Value;
        assert!(lhs.ty() == rhs.ty());

        Div {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn ty(&self) -> ir::Type { self.lhs.ty() }
}

impl_instruction!(Div: lhs, rhs);

