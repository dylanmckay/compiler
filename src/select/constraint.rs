use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Constraint
{
    /// Constraints that two operands are identical.
    Equality(util::Id, util::Id),
}

