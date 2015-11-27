use ir::{self,Instruction,Expression};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Sub
{
    lhs: Box<ir::Expression>,
    rhs: Box<ir::Expression>,
}

impl Sub
{
    pub fn new(lhs: ir::Expression, rhs: ir::Expression) -> Self {
        assert!(lhs.ty() == rhs.ty());

        Sub {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn ty(&self) -> ir::Type { self.lhs.ty() }
}

impl_instruction!(Sub: lhs, rhs);
