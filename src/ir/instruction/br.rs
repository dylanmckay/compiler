
use ir::{self,Instruction,Value};
use std::fmt;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Break
{
    cond: ir::Condition,
    target: Box<ir::Value>,
}

impl Break
{
    /// Creates a conditional branch.
    pub fn conditional(cond: ir::Condition,
                       target: ir::Value) -> Self {
        Break {
            cond: cond,
            target: Box::new(target),
        }
    }

    /// Creates an unconditional branch.
    pub fn unconditional(target: ir::Value) -> Self {
        Break::conditional(ir::Condition::True, target)
    }

    pub fn ty(&self) -> ir::Type { ir::Type::void() }
}

impl_instruction!(Break: target);

