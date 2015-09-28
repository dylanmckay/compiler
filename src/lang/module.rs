
use std::{self,fmt};
use lang;

/// An SSA module.
/// 
/// A module is made up of functions, basic blocks, and instructions
/// in **S**ingle **S**tatic **A**ssignment form.
pub trait Module : fmt::Display
{
    type Function: lang::Function;

    fn functions<'a>(&'a self) -> std::slice::Iter<'a,Self::Function>;
    fn functions_mut<'a>(&'a mut self) -> std::slice::IterMut<'a,Self::Function>;

    fn map_functions<F>(mut self, f: F) -> Self
        where F: FnMut(Self::Function) -> Self::Function;

    fn with_functions<I>(self, funcs: I) -> Self
        where I: Iterator<Item=Self::Function>;
}

