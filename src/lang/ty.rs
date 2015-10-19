
use std::fmt;

pub trait Type : Clone + fmt::Display + fmt::Debug + PartialEq + Eq
{
}
