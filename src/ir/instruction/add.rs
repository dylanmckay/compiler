use ir::{self,Instruction,Value,Expression};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Add
{
    lhs: Box<ir::Value>,
    rhs: Box<ir::Value>,
}

impl Add
{
    pub fn new(lhs: ir::Value, rhs: ir::Value) -> Self {
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

