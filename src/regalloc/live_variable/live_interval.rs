use Instruction;
use LiveRange;

/// An interval a value is defined over.
pub struct LiveInterval<I: Instruction>
{
    /// The range the variable is live for. 
    pub range: LiveRange,

    /// The register class the interval is restricted to.
    pub register_class: I::RegisterClass,
    /// The optionally selected register.
    pub register: Option<I::Register>,
}

/// A collection of live intervals.
pub struct LiveIntervals<I: Instruction>
{
    pub intervals: Vec<LiveInterval<I>>,
}

