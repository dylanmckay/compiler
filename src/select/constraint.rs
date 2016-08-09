use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Constraint
{
    /// Constraints that two operands are identical.
    Equality {
        constrained_register_ids: Vec<util::Id>,
    }
}

