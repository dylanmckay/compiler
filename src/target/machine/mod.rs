use Target;

pub mod avr;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Register
{
    name: &'static str,
    number: u32,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct RegisterClass
{
    name: &'static str,
    registers: &'static [&'static Register],
}

/// A target.
pub trait MachineTarget : Target
{
    /// Gets the width of a pointer.
    fn pointer_width(&self) -> u16;

    /// Gets the register classes the target supports.
    fn register_classes(&self) -> &'static [&'static RegisterClass];
}
