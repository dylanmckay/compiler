pub use self::machine::{RegisterClass,MachineTarget};

pub mod machine;

/// A target.
pub trait Target
{
    fn name(&self) -> &'static str;
}

