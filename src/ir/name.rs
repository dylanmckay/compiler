
use std::fmt;

/// Represents a name.
#[derive(Clone)]
pub enum Name
{
    /// The value is unnamed.
    Unnamed,
    /// The value has a name.
    Named(String),
}

impl Name
{
    /// Create an unspecified name.
    pub fn unnamed() -> Name {
        Name::Unnamed
    }

    pub fn named<S>(name: S) -> Name
        where S: Into<String> {

        Name::Named(name.into())
    }
}

impl fmt::Display for Name
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        match self {
            &Name::Unnamed => unimplemented!(), // FIXME: we need to have a global accumulator
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
