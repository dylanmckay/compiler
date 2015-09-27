
pub mod manager;
pub use self::manager::Manager;

use lang;

/// A pass identifier.
pub type Id = u64;

pub enum PassKind<M: lang::Module>
{
    Immutable(Box<Pass<M>>),
    Mutable(Box<PassMut<M>>),
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

    /// Checks if the pass is mutable.
    fn is_mutable(&self) -> bool { false }
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

    fn is_mutable(&self) -> bool { true }
}

impl<M> Into<PassKind<M>> for Box<Pass<M>>
    where M: lang::Module {

    fn into(self) -> PassKind<M> {
        PassKind::Immutable(self)
    }
}

impl<M> Into<PassKind<M>> for Box<PassMut<M>>
    where M: lang::Module {

    fn into(self) -> PassKind<M> {
        PassKind::Mutable(self)
    }
}
