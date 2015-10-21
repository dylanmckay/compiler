
use lang;
use util;

use lang::{Value,Function,Global};

use std;

/// An IR module.
#[derive(Clone,Debug)]
pub struct Module<V: Value>
{
    functions: util::Set<Function<V>>,
    globals: util::Set<Global<V>>,
}

impl<V> Module<V>
    where V: lang::Value
{
    /// Creates an empty module.
    pub fn empty() -> Self {
        Module {
            functions: util::Set::empty(),
            globals: util::Set::empty(),
        }
    }

    pub fn flatten(self) -> Self {
        self.map_functions(|f| f.flatten())
    }

    /// Adds a function to the module.
    pub fn function(mut self, func: Function<V>) -> Self {
        self.functions.add(func);
        self
    }

    /// Adds a global to the module.
    pub fn global(mut self, global: Global<V>) -> Self {
        self.globals.add(global);
        self
    }

    pub fn functions<'a>(&'a self) -> std::slice::Iter<'a,Function<V>> {
        self.functions.iter()
    }

    pub fn map_functions<F>(mut self, mut f: F) -> Self
        where F: FnMut(Function<V>) -> Function<V> {

        let funcs = self.functions.into_iter().map(|a| f(a));
        self.functions = funcs.collect();

        self
    }

    pub fn globals<'a>(&'a self) -> std::slice::Iter<'a,Global<V>> {
        self.globals.iter()
    }

    pub fn map_globals<F>(mut self, mut f: F) -> Self
        where F: FnMut(Global<V>) -> Global<V> {

        let globals = self.globals.into_iter().map(|a| f(a));
        self.globals = globals.collect();

        self
    }
}

