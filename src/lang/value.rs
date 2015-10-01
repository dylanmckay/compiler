
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
    fn is_single_critical(&self) -> bool {
        true // conservately mark all values critical by default
    }

    /// Checks if a value is critical.
    ///
    /// Recursively checks whether this value is critical.
    fn is_critical(&self) -> bool {
        self::is_critical_recursive(self)
    }
}

fn is_critical_recursive<V: Value>(value: &V) -> bool {

    let subvalues_critical = value.subvalues().iter().any(|a| is_critical_recursive(a));

    value.is_single_critical() || subvalues_critical
}
