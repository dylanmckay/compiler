use Target;

/// A target.
pub trait MachineTarget : Target
{
    /// Gets the width of a pointer.
    fn pointer_width(&self) -> u16;

    /// Gets the register classes the target supports.
    fn register_classes(&self) -> &'static [&'static RegisterClass];
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Register
{
    pub name: &'static str,
    pub number: u32,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct RegisterClass
{
    pub name: &'static str,
    pub registers: &'static [&'static Register],
}
