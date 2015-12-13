use {Value,Function,Global,Block};
use util::Identifiable;
use util;

use std;

/// A module.
#[derive(Clone,Debug)]
pub struct Module<V: Value>
{
    functions: util::List<Function<V>>,
    globals: util::List<Global<V>>,
}

impl<V> Module<V>
    where V: Value
{
    /// Creates an empty module.
    pub fn empty() -> Self {
        Module {
            functions: util::List::empty(),
            globals: util::List::empty(),
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

    /// Adds a function to the module.
    pub fn add_function(&mut self, func: Function<V>) {
        self.functions.add(func);
    }

    /// Adda a global to the module.
    pub fn add_global(&mut self, global: Global<V>) {
        self.globals.add(global);
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
        self.find_global(id)
            .expect("no global with that ID exists, or that global is locked")
    }

    /// Finds a function by its ID.
    pub fn find_function(&self, id: util::Id) -> util::Slot<&Function<V>> {
        self.functions.lookup(id)
    }

    /// Gets a function from its ID, `panic`ing if it does not exist.
    pub fn get_function(&self, id: util::Id) -> &Function<V> {
        self.find_function(id)
            .expect("no function with that ID exists, or that function is locked")
    }

    /// Finds a block by its ID.
    pub fn find_block(&self, id: util::Id) -> Option<&Block<V>> {
        self.functions().flat_map(|f| f.blocks())
                        .find(|b| b.get_id() == id)
    }

    /// Gets a block from its ID, `panic`ing if it does not exist.
    pub fn get_block(&self, id: util::Id) -> &Block<V> {
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

    pub fn map_values<F>(mut self, mut f: F) -> Self
        where F: FnMut(V) -> V {

        self.globals = self.globals.into_iter()
                                   .map(|g| g.map_value(|v| f(v)))
                                   .collect();
        self.functions = self.functions.into_iter()
                                       .map(|a| a.map_values(|v| f(v)))
                                       .collect();
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

impl<V> Extend<Global<V>> for Module<V>
    where V: Value
{
    fn extend<I>(&mut self, it: I)
        where I: IntoIterator<Item=Global<V>> {
        self.globals.extend(it)
    }
}

impl<V> Extend<Function<V>> for Module<V>
    where V: Value
{
    fn extend<I>(&mut self, it: I)
        where I: IntoIterator<Item=Function<V>> {
        self.functions.extend(it)
    }
}
