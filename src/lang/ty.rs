
use std::fmt;

pub trait Type : fmt::Display + PartialEq + Eq
{
}
