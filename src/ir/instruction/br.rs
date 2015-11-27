
use ir::{self,Instruction,Expression};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Break
{
    cond: ir::Condition,
    target: Box<ir::Expression>,
}

impl Break
{
    /// Creates a conditional branch.
    pub fn conditional(cond: ir::Condition,
                       target: ir::Expression) -> Self {
        Break {
            cond: cond,
            target: Box::new(target),
        }
    }

    /// Creates an unconditional branch.
    pub fn unconditional(target: ir::Expression) -> Self {
        Break::conditional(ir::Condition::True, target)
    }

    pub fn condition(&self) -> &ir::Condition {
        &self.cond
    }

    pub fn target(&self) -> &ir::Expression {
        &self.target
    }

    pub fn ty(&self) -> ir::Type { ir::Type::void() }
}

impl_instruction!(Break: target);

