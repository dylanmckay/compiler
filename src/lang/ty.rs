
use std::fmt;

/// A type.
pub trait Type : Clone + fmt::Display +
                 fmt::Debug +
                 PartialEq + Eq
{
}
