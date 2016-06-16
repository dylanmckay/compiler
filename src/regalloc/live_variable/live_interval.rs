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

