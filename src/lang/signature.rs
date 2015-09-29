
use lang;
use std::fmt;

/// A function signature.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Signature<T: lang::Type>
{
    pub return_types: Vec<T>,
    pub param_types: Vec<T>,
}

impl<T: lang::Type> Signature<T>
{
    pub fn new() -> Self {
        Signature {
            return_types: Vec::new(),
            param_types: Vec::new(),
        }
    }

    pub fn ret(mut self, ty: T) -> Self {
        self.return_types.push(ty);
        self
    }

    pub fn param(mut self, ty: T) -> Self {
        self.param_types.push(ty);
        self
    }
}

impl<T: lang::Type> fmt::Display for Signature<T>
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

impl<T: lang::Type> lang::Type for Signature<T> { }


