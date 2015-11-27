use ir::{self,Instruction,Expression};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Add
{
    lhs: Box<ir::Expression>,
    rhs: Box<ir::Expression>,
}

impl Add
{
    pub fn new(lhs: ir::Expression, rhs: ir::Expression) -> Self {
        assert!(lhs.ty() == rhs.ty());

        Add {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn ty(&self) -> ir::Type {
        self.lhs.ty()
    }
}

impl_instruction!(Add: lhs, rhs);

