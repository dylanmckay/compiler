
use std::{self,fmt};
use lang;

pub trait Module : fmt::Display
{
    type Function: lang::Function;

    fn functions<'a>(&'a self) -> std::slice::Iter<'a,Self::Function>;
}
