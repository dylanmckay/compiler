
use ir::types::{Type,TypeTrait};
use std;

/// A function signature in IR.
#[derive(Clone,Eq,PartialEq,Debug)]
pub struct Function
{
    param_types: Vec<Type>,
    return_types: Vec<Type>,
}

impl Function
{
    pub fn new() -> Self {
        Function {
            param_types: Vec::new(),
            return_types: Vec::new(),
        }
    }

    pub fn ret(mut self, value: Type) -> Self {
        self.return_types.push(value);
        self
    }

    pub fn param(mut self, value: Type) -> Self {
        self.param_types.push(value);
        self
    }

    pub fn returns<'a>(&'a self) -> std::slice::Iter<'a,Type> {
        self.return_types.iter()
    }

    pub fn parameters<'a>(&'a self) -> std::slice::Iter<'a,Type> {
        self.param_types.iter()
    }
}

impl TypeTrait for Function { }

impl std::fmt::Display for Function
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use util;

        write!(fmt, "{} ({})", 
               util::comma_separated_values(self.returns()),
               util::comma_separated_values(self.parameters()))

    }
}

impl_type!(Function);
