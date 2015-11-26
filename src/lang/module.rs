
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
        self.map_functions(|f,_| f.flatten())
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
    pub fn find_global(&self, id: util::Id) -> util::Slot<&Global<V>> {
        self.globals.lookup(id)
    }

    /// Gets a global from its ID, `panic`ing if it does not exist.
    pub fn get_global(&self, id: util::Id) -> &Global<V> {
        self.find_global(id).expect("no global with that ID exists")
    }

    /// Finds a function by its ID.
    pub fn find_function(&self, id: util::Id) -> util::Slot<&Function<V>> {
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

    /// Gets the functions that the module contains as mutable.
    pub fn functions_mut(&mut self) -> std::slice::IterMut<Function<V>> {
        self.functions.iter_mut()
    }

    /// Performs a mapping over the functions that the module contains.
    pub fn map_functions<F>(mut self, mut f: F) -> Self
        where F: FnMut(Function<V>, &Self) -> Function<V> {

        // Here we unsafely get a reference to the module so that
        // it persists after we mutably borrow the module while mapping.
        //
        // The 'Set' class will recognize whatever function is being mapped,
        // and not allow it to be looked up while mapping.
        //
        // This allows us to map the functions in a module while
        // having immutable access to the rest of the module safely.
        let self_ref: &Self = unsafe {
            ::std::mem::transmute(&self as *const Self)
        };

        self.functions.map_in_place(|func| f(func, self_ref));
        self
    }

    /// Gets the globals that the module contains.
    pub fn globals(&self) -> std::slice::Iter<Global<V>> {
        self.globals.iter()
    }

    /// Gets the globals that the module contains as mutable.
    pub fn globals_mut(&mut self) -> std::slice::IterMut<Global<V>> {
        self.globals.iter_mut()
    }

    /// Performs a mapping over the global variables that the module contains.
    pub fn map_globals<F>(mut self, f: F) -> Self
        where F: FnMut(Global<V>) -> Global<V> {

        let globals = self.globals.into_iter().map(f);
        self.globals = globals.collect();

        self
    }

    pub fn values(&self) -> std::vec::IntoIter<&V> {
        // FIXME: use 'impl Iterator' once supported
        let vals: Vec<_> = self.globals.iter()
                                       .map(|g| g.value())
                                       .chain(self.functions.iter()
                                                            .flat_map(Function::values))
                                       .collect();
        vals.into_iter()
    }
}

