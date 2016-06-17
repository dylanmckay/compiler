use {LiveRange, Target};

/// An interval a value is defined over.
pub struct LiveInterval<T: Target>
{
    /// The range the variable is live for. 
    pub range: LiveRange,

    /// The register class the interval is restricted to.
    pub register_class: T::RegisterClass,
    /// The optionally selected register.
    pub register: Option<T::Register>,
}

/// A collection of live intervals.
pub struct LiveIntervals<T: Target>
{
    pub intervals: Vec<LiveInterval<T>>,
}

