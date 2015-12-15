use Target;
use std;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct RegisterClass
{
    name: String,
    size: u16,
}

/// A target.
pub trait MachineTarget : Target
{
    /// Gets the width of a pointer.
    fn pointer_width(&self) -> u16;

    /// Gets the register classes the target supports.
    fn register_classes(&self) -> std::slice::Iter<RegisterClass>;
}
