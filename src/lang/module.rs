
use lang;
use util;

use lang::{Value,Function,Global};

use std;

/// A module.
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

    /// Flattens the module so that it is no longer in
    /// SSA-form (if it was already).
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

    /// Finds a global by its ID.
    pub fn find_global(&self, id: util::Id) -> Option<&Global<V>> {
        self.globals.lookup(id)
    }

    /// Gets a global from its ID, `panic`ing if it does not exist.
    pub fn get_global(&self, id: util::Id) -> &Global<V> {
        self.find_global(id).expect("no global with that ID exists")
    }

    /// Finds a function by its ID.
    pub fn find_function(&self, id: util::Id) -> Option<&Function<V>> {
        self.functions.lookup(id)
    }

    /// Gets a function from its ID, `panic`ing if it does not exist.
    pub fn get_function(&self, id: util::Id) -> &Function<V> {
        self.find_function(id).expect("no function with that ID exists")
    }

    /// Finds a block by its ID.
    pub fn find_block(&self, id: util::Id) -> Option<&lang::Block<V>> {
        self.functions().flat_map(|f| f.blocks())
                        .find(|b| b.id() == id)
    }

    /// Gets a block from its ID, `panic`ing if it does not exist.
    pub fn get_block(&self, id: util::Id) -> &lang::Block<V> {
        self.find_block(id).expect("no block with that ID exists")
    }

    /// Gets the functions that the module contains.
    pub fn functions(&self) -> std::slice::Iter<Function<V>> {
        self.functions.iter()
    }

    /// Performs a mapping over the functions that the module contains.
    pub fn map_functions<F>(mut self, f: F) -> Self
        where F: FnMut(Function<V>) -> Function<V> {

        let funcs = self.functions.into_iter().map(f);
        self.functions = funcs.collect();

        self
    }

    /// Gets the globals that the module contains.
    pub fn globals(&self) -> std::slice::Iter<Global<V>> {
        self.globals.iter()
    }

    /// Performs a mapping over the global variables that the module contains.
    pub fn map_globals<F>(mut self, f: F) -> Self
        where F: FnMut(Global<V>) -> Global<V> {

        let globals = self.globals.into_iter().map(f);
        self.globals = globals.collect();

        self
    }
}

