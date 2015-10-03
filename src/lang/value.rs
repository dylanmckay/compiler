
use std::fmt;

/// A value.
pub trait Value : Sized + fmt::Display
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

    /// Checks if a single value is critical.
    ///
    /// If a value contains another critical value, it is also a
    /// critical value. Critical values are not removed when dead
    /// code elimination is run.
    ///
    /// This does not check if subvalues are critical.
    // TODO: Find a better name.
    fn is_single_critical(&self) -> bool {
        true // conservately mark all values critical by default
    }

    /// Checks if a value is critical.
    ///
    /// Recursively checks whether this value is critical.
    fn is_critical(&self) -> bool {
        let subvalues_critical = self.subvalues().iter().any(|a| a.is_critical());

        self.is_single_critical() || subvalues_critical
    }
}

