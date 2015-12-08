use std;

/// Represents a name.
#[derive(Clone,Debug)]
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

    pub fn is_named(&self) -> bool {
        match *self {
            Name::Unnamed => false,
            Name::Named(..) => true,
        }
    }
}

impl std::fmt::Display for Name
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match *self {
            Name::Unnamed => "unnamed".fmt(fmt), // FIXME: we need to have a global accumulator
            Name::Named(ref val) => val.fmt(fmt),
        }
    }
}

impl std::cmp::PartialEq for Name
{
    fn eq(&self, other: &Name) -> bool {
         match (self,other) {
             (&Name::Named(ref n1),
              &Name::Named(ref n2)) => n1 == n2,
              _ => false, // unnamed values are always unique
         }
    }
}

impl std::cmp::Eq for Name { }
