pub use self::manager::Manager;

/// The pass manager infrastructure.
pub mod manager;
/// Passes which perform transformations.
pub mod transforms;
/// Passes which perform analysis.
pub mod analysis;
/// The pass registry.
pub mod registrar;

extern crate compiler_ir as ir;
extern crate compiler_util as util;

extern crate num;

/// A pass identifier.
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Id(u32);

pub enum Info
{
    Analysis(Box<Analysis>),
    Transform(Box<Transform>),
}

impl Metadata for Info
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

pub trait Analysis : Metadata
{
    /// Run the pass on an entire module.
    fn run_module(&mut self,
                  module: &ir::Module) {

        for global in module.globals() {
            self.run_global(global);
        }

        for func in module.functions() {
            self.run_function(func);
        }
    }

    /// Run the pass on a global variable.
    fn run_global(&mut self,
                  _: &ir::Global) {
        // do nothing unless overidden
    }

    /// Run the pass on a function.
    fn run_function(&mut self,
                    function: &ir::Function) {

        for bb in function.blocks() {
            self.run_block(bb);
        }
    }

    /// Run the pass on a basic block.
    fn run_block(&mut self,
                 block: &ir::Block) {

        for value in block.values() {
            self.run_value_recursive(&value);
        }
    }

    /// Run the pass on a value.
    fn run_value(&mut self,
                 _: &ir::Value) {

        panic!("the pass is not implemented");
    }

    fn run_value_recursive(&mut self,
                           value: &ir::Value) {

        // Recurse from the deepest node to the root node.
        for val in value.node.subvalues() {
            self.run_value_recursive(&val);
        }

        self.run_value(value)
    }
}

pub trait Transform : Metadata
{
    /// Run the pass on an entire module.
    fn run_module(&mut self,
                  module: ir::Module) -> ir::Module {
        module.map_globals(|a| self.run_global(a))
              .map_functions(|a,module| self.run_function(a,module))
    }

    /// Run the pass on a global.
    fn run_global(&mut self,
                  global: ir::Global)
        -> ir::Global {

        // do nothing unless overridden
        global
    }

    /// Run the pass on a function.
    fn run_function(&mut self,
                    function: ir::Function,
                    _module: &ir::Module)
        -> ir::Function {

        function.map_blocks(|a| self.run_block(a))
    }

    /// Run the pass on a basic block.
    fn run_block(&mut self,
                 block: ir::Block)
        -> ir::Block {

        block.map_values(|a| self.run_value_recursive(a))
    }

    /// Run the pass on a value.
    fn run_value(&mut self,
                 _: ir::Value) -> ir::Value {

        panic!("the {} pass is not implemented", self.name());
    }

    fn run_value_recursive(&mut self,
                           value: ir::Value) -> ir::Value {

        let val = ir::Value {
            // Recurse from the deepest node to the root node.
            node: value.node.map_subvalues(|v| self.run_value_recursive(v)),
        };

        self.run_value(val)
    }
}

