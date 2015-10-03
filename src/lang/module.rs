
use std::{self,fmt};
use lang;

/// An SSA module.
/// 
/// A module is made up of functions, basic blocks, and instructions
/// in **S**ingle **S**tatic **A**ssignment form.
pub trait Module : fmt::Display
{
    type Function: lang::Function;
    type Global;

    /// Gets the functions that the module contains.
    fn functions<'a>(&'a self) -> std::slice::Iter<'a,Self::Function>;

    /// Maps the functions.
    fn map_functions<F>(self, f: F) -> Self
        where F: FnMut(Self::Function) -> Self::Function;

    /// Gets the globals that the module contains.
    fn globals<'a>(&'a self) -> std::slice::Iter<'a,Self::Global>;

    /// Maps the globals.
    fn map_globals<F>(self, f: F) -> Self
        where F: FnMut(Self::Global) -> Self::Global;
}

