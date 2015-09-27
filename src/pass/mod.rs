
pub use self::manager::Manager;

pub mod manager;
pub mod transforms;


use lang;

/// A pass identifier.
pub type Id = u64;

pub enum Info<M: lang::Module>
{
    Immutable(Box<Pass<M>>),
    Mutable(Box<PassMut<M>>),
}

impl<M: lang::Module> Metadata for Info<M>
{
    fn id(&self) -> Id {
        match self {
            &Info::Immutable(ref p) => p.id(),
            &Info::Mutable(ref p) => p.id(),
        }
    }

    fn dependencies(&self) -> &'static [Id] {
        match self {
            &Info::Immutable(ref p) => p.dependencies(),
            &Info::Mutable(ref p) => p.dependencies(),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            &Info::Immutable(ref p) => p.name(),
            &Info::Mutable(ref p) => p.name(),
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

pub trait Pass<M> : Metadata
    where M: lang::Module
{
    fn run_module(&mut self, module: &M) {

        for func in module.functions() {
            self.run_function(func);
        }
    }

    fn run_function(&mut self, function: &M::Function) {
        use lang::Function;
        
        for bb in function.basic_blocks() {
            self.run_block(bb);
        }
    }

    fn run_block(&mut self, _: &<M::Function as lang::Function>::BasicBlock) {
        panic!("the pass is not implemented");
    }
}

pub trait PassMut<M> : Metadata
    where M: lang::Module
{
    fn run_module(&mut self, module: &mut M) {

        for func in module.functions_mut() {
            self.run_function(func);
        }
    }

    fn run_function(&mut self, function: &mut M::Function) {
        use lang::Function;

        for bb in function.basic_blocks_mut() {
            self.run_block(bb);
        }
    }

    fn run_block(&mut self, _: &mut <M::Function as lang::Function>::BasicBlock) {
        panic!("the pass is not implemented");
    }
}

impl<M> Into<Info<M>> for Box<Pass<M>>
    where M: lang::Module {

    fn into(self) -> Info<M> {
        Info::Immutable(self)
    }
}

impl<M> Into<Info<M>> for Box<PassMut<M>>
    where M: lang::Module {

    fn into(self) -> Info<M> {
        Info::Mutable(self)
    }
}
