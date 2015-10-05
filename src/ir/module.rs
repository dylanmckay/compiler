
use ir::{Function,Global};
use lang;
use ir;
use util;

use std;

/// An IR module.
pub struct Module
{
    functions: util::Set<Function>,
    globals: util::Set<Global>,
}

impl Module
{
    /// Creates an empty module.
    pub fn empty() -> Self {
        Module {
            functions: util::Set::empty(),
            globals: util::Set::empty(),
        }
    }

    /// Adds a function to the module.
    pub fn function(mut self, func: Function) -> Self {
        self.functions.add(func);
        self
    }

    /// Adds a global to the module.
    pub fn global(mut self, global: Global) -> Self {
        self.globals.add(global);
        self
    }
}

impl lang::Module for Module {
    type Function = Function;
    type Global = Global;

    fn functions<'a>(&'a self) -> std::slice::Iter<'a,Function> {
        self.functions.iter()
    }

    fn map_functions<F>(mut self, mut f: F) -> Self
        where F: FnMut(Function) -> Function {

        let funcs = self.functions.into_iter().map(|a| f(a));
        self.functions = funcs.collect();

        self
    }

    fn globals<'a>(&'a self) -> std::slice::Iter<'a,Global> {
        self.globals.iter()
    }

    fn map_globals<F>(mut self, mut f: F) -> Self
        where F: FnMut(Global) -> Global {

        let globals = self.globals.into_iter().map(|a| f(a));
        self.globals = globals.collect();

        self
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        ir::print::module(self, fmt)
    }
}
