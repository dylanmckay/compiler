
pub mod manager;
pub use self::manager::Manager;

use lang;

/// A pass identifier.
pub type Id = u64;

pub enum PassInfo<M: lang::Module>
{
    Immutable(Box<Pass<M>>),
    Mutable(Box<PassMut<M>>),
}

impl<M: lang::Module> PassMetadata for PassInfo<M>
{
    fn id(&self) -> Id {
        match self {
            &PassInfo::Immutable(ref p) => p.id(),
            &PassInfo::Mutable(ref p) => p.id(),
        }
    }

    fn dependencies(&self) -> &'static [Id] {
        match self {
            &PassInfo::Immutable(ref p) => p.dependencies(),
            &PassInfo::Mutable(ref p) => p.dependencies(),
        }
    }
}

/// A pass over a set of instructions.
pub trait PassMetadata
{
    fn id(&self) -> Id;
    
    /// Gets the identifiers of the passes this pass
    /// depends on.
    fn dependencies(&self) -> &'static [Id] {
        &[]
    }
}

pub trait Pass<M> : PassMetadata
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

    fn run_block(&mut self, block: &<M::Function as lang::Function>::BasicBlock) {
        panic!("the pass is not implemented");
    }
}

pub trait PassMut<M> : PassMetadata
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

    fn run_block(&mut self, block: &mut <M::Function as lang::Function>::BasicBlock) {
        panic!("the pass is not implemented");
    }
}

impl<M> Into<PassInfo<M>> for Box<Pass<M>>
    where M: lang::Module {

    fn into(self) -> PassInfo<M> {
        PassInfo::Immutable(self)
    }
}

impl<M> Into<PassInfo<M>> for Box<PassMut<M>>
    where M: lang::Module {

    fn into(self) -> PassInfo<M> {
        PassInfo::Mutable(self)
    }
}
