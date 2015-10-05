
use ir::Function;
use ir::Global;
use lang;
use ir;
use util;

use std;

/// An IR module.
pub struct Module
{
    functions: Vec<Function>,
    globals: Vec<Global>,

    generator: util::id::Generator,
}

impl Module
{
    /// Creates an empty module.
    pub fn empty() -> Self {
        Module {
            functions: Vec::new(),
            globals: Vec::new(),

            generator: util::id::Generator::new(),
        }
    }

    /// Adds a function to the module.
    pub fn function(mut self,mut  func: Function) -> Self {
        // assign an ID to the function
        func.set_id(self.generator.next());

        self.functions.push(func);
        self
    }

    /// Adds a global to the module.
    pub fn global(mut self, global: Global) -> Self {
        self.globals.push(global);
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
