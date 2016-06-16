use util;

/// A range of instructions which may have live variables.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct LiveRange
{
    /// The ID of the first instruction in the range.
    pub start_id: util::Id,
    /// The ID of the last instruction in the range.
    pub end_id: util::Id,
}

