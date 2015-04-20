
use std::fmt;

/// An instruction.
pub trait Instruction : Sized + fmt::Display
{
    fn map<F,T>(self, f: F) -> T
        where F: Fn(Self) -> T,
              T: Instruction {
        f(self)
    }
}
