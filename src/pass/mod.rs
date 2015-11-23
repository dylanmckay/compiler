
pub use self::manager::Manager;

/// The pass manager infrastructure.
pub mod manager;
/// Passes which perform transformations.
pub mod transforms;
/// Passes which perform analysis.
pub mod analysis;


use lang;

/// A pass identifier.
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Id(u32);

pub enum Info<V: lang::Value>
{
    Analysis(Box<Analysis<V>>),
    Transform(Box<Transform<V>>),
}

impl<V: lang::Value> Metadata for Info<V>
{
    fn id(&self) -> Id {
        match *self {
            Info::Analysis(ref p) => p.id(),
            Info::Transform(ref p) => p.id(),
        }
    }

    fn dependencies(&self) -> &'static [Id] {
        match *self {
            Info::Analysis(ref p) => p.dependencies(),
            Info::Transform(ref p) => p.dependencies(),
        }
    }

    fn name(&self) -> &'static str {
        match *self {
            Info::Analysis(ref p) => p.name(),
            Info::Transform(ref p) => p.name(),
        }
    }
}

/// A pass over a set of instructions.
pub trait Metadata
{
    fn id(&self) -> Id;

    /// Gets the identifiers of the passes this pass
    /// depends on.
    fn dependencies(&self) -> &'static [Id] {
        &[]
    }

    fn name(&self) -> &'static str;
}

pub trait Analysis<V> : Metadata
    where V: lang::Value
{
    /// Run the pass on an entire module.
    fn run_module(&mut self,
                  module: &lang::Module<V>) {

        for global in module.globals() {
            self.run_global(global);
        }

        for func in module.functions() {
            self.run_function(func);
        }
    }

    /// Run the pass on a global variable.
    fn run_global(&mut self,
                  _: &lang::Global<V>) {
        // do nothing unless overidden
    }

    /// Run the pass on a function.
    fn run_function(&mut self,
                    function: &lang::Function<V>) {

        for bb in function.blocks() {
            self.run_block(bb);
        }
    }

    /// Run the pass on a basic block.
    fn run_block(&mut self,
                 block: &lang::Block<V>) {

        for value in block.values() {
            self.run_value_recursive(&value);
        }
    }

    /// Run the pass on a value.
    fn run_value(&mut self,
                 _: &V) {

        panic!("the pass is not implemented");
    }

    fn run_value_recursive(&mut self,
                           value: &V) {

        // Recurse from the deepest node to the root node.
        for val in value.subvalues() {
            self.run_value_recursive(&val);
        }

        self.run_value(value)
    }
}

pub trait Transform<V> : Metadata
    where V: lang::Value
{
    /// Run the pass on an entire module.
    fn run_module(&mut self,
                  module: lang::Module<V>) -> lang::Module<V> {
        module.map_globals(|a| self.run_global(a))
              .map_functions(|a| self.run_function(a))
    }

    /// Run the pass on a global.
    fn run_global(&mut self,
                  global: lang::Global<V>)
        -> lang::Global<V> {

        // do nothing unless overridden
        global
    }

    /// Run the pass on a function.
    fn run_function(&mut self,
                    function: lang::Function<V>)
        -> lang::Function<V> {

        function.map_blocks(|a| self.run_block(a))
    }

    /// Run the pass on a basic block.
    fn run_block(&mut self,
                 block: lang::Block<V>)
        -> lang::Block<V> {

        block.map_values(|a| self.run_value_recursive(a))
    }

    /// Run the pass on a value.
    fn run_value(&mut self,
                 _: V) -> V {

        panic!("the {} pass is not implemented", self.name());
    }

    fn run_value_recursive(&mut self,
                           value: V) -> V {

        // Recurse from the deepest node to the root node.
        let val = value.map_subvalues(|v| self.run_value_recursive(v));
        self.run_value(val)
    }
}

