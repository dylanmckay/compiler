
use ir::{self,Instruction,Expression};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Mul
{
    pub lhs: Box<ir::Expression>,
    pub rhs: Box<ir::Expression>,
}

impl Mul
{
    pub fn new(lhs: ir::Expression, rhs: ir::Expression) -> Self {
        Mul {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn ty(&self) -> ir::Type { self.lhs.ty() }
}

impl_instruction!(Mul: lhs, rhs);
