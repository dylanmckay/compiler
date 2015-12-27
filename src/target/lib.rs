pub use self::machine::{RegisterClass,MachineTarget};

pub mod machine;

/// The AVR target.
pub mod avr;

/// A target.
pub trait Target
{
    fn name(&self) -> &'static str;
}

