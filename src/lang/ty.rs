
use std::fmt;

pub trait Type : Clone + fmt::Display + PartialEq + Eq
{
}
