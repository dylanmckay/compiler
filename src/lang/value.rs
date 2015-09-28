
use std::fmt;

/// A value.
pub trait Value : Sized + fmt::Debug + fmt::Display
{
    /// Gets the set of values.
    /// TODO: make this an iterator once supported.
    fn subvalues(&self) -> Vec<Self>;

    /// Maps values to other values.
    fn map_subvalues<F>(self, f: F) -> Self
        where F: FnMut(Self) -> Self;

    fn map<F,T>(self, f: F) -> T
        where F: Fn(Self) -> T {
        f(self)
    }

}
