
/// Machine-code based targets.
pub mod mc;

use std;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct RegisterClass
{
    name: String,
    size: u16,
}

/// A target.
pub trait Target
{
    fn name(&self) -> &'static str;
}

/// A target.
pub trait MachineTarget : Target
{
    /// Gets the width of a pointer.
    fn pointer_width(&self) -> u16;

    /// Gets the register classes the target supports.
    fn register_classes<'a>(&'a self) -> std::slice::Iter<'a,RegisterClass>;
}

