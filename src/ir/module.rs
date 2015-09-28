
use ir::Function;
use lang;
use std::{self,fmt};

/// An IR module.
pub struct Module
{
    functions: Vec<Function>,
}

impl Module
{
    /// Creates an empty module.
    pub fn empty() -> Self {
        Module {
            functions: Vec::new(),
        }
    }

    /// Adds a function to the module.
    pub fn function(mut self, func: Function) -> Self {
        self.functions.push(func);

        self
    }
}

impl lang::Module for Module {
    type Function = Function;

    fn functions<'a>(&'a self) -> std::slice::Iter<'a,Function> {
        self.functions.iter()
    }

    fn functions_mut<'a>(&'a mut self) -> std::slice::IterMut<'a,Function> {
        self.functions.iter_mut()
    }

    fn map_functions<F>(mut self, mut f: F) -> Self
        where F: FnMut(Function) -> Function {

        let funcs = self.functions.into_iter().map(|a| f(a));
        self.functions = funcs.collect();

        self
    }

    fn with_functions<I>(mut self, funcs: I) -> Self
        where I: Iterator<Item=Function> {

        self.functions = funcs.collect();
        self
    }
}

impl fmt::Display for Module {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {

        for func in self.functions.iter() {
            try!(write!(fmt, "{}\n", func));
        }

        Ok(())
    }
}
