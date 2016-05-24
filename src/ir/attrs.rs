
/// The calling convention.
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum CallingConvention
{
    /// The C calling convention.
    C,
}

impl Default for CallingConvention
{
    fn default() -> Self {
        CallingConvention::C
    }
}

/// Specifies the inline hint.
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum InlineHint
{
    /// No inline hint specified.
    None,
    /// The function should be inlined.
    Inline,
    /// The function **must** be inlined.
    MustInline
}

impl Default for InlineHint
{
    fn default() -> Self {
        InlineHint::None
    }
}

/// Specifies constraints on the complexity of the generated code.
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum ComplexityHint
{
    /// The code generator is free to generate code of any complexity.
    None,
    /// The code generated must run in constant time w.r.t inputs.
    ConstantTime,
}

impl Default for ComplexityHint
{
    fn default() -> Self {
        ComplexityHint::None
    }
}
