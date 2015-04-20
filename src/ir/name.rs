
use std::fmt;

/// Represents a name.
#[derive(Clone)]
pub enum Name
{
    /// The value is unnamed and is represented as
    /// an unsigned numeric value.
    Unnamed(u64),
    /// The value has a name.
    Named(String),
}

impl Name
{
    pub fn unnamed(val: u64) -> Name {
        Name::Unnamed(val)
    }

    pub fn named(name: String) -> Name {
        Name::Named(name.to_owned())
    }
}

impl fmt::Display for Name
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        match self {
            &Name::Unnamed(val) => val.fmt(fmt),
            &Name::Named(ref val) => val.fmt(fmt),
        }
    }
}

impl fmt::Debug for Name
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        fmt::Display::fmt(self, fmt)
    }
}
