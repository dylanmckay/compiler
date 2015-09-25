
pub mod manager;
pub use self::manager::Manager;

use lang;

/// A pass identifier.
pub type Id = u64;

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

pub trait Pass : PassMetadata
{
    fn run_module<M>(&mut self, module: &M)
        where M: lang::Module {

        for func in module.functions() {
            self.run_function(func);
        }
    }

    fn run_function<F>(&mut self, function: &F)
        where F: lang::Function {
        
        for bb in function.basic_blocks() {
            self.run_block(bb);
        }
    }

    fn run_block<B>(&mut self, block: &B)
        where B: lang::BasicBlock {
        panic!("the pass is not implemented");
    }
}

pub trait PassMut : PassMetadata
{
    fn run_module<M>(&mut self, module: &mut M)
        where M: lang::Module {

        for func in module.functions_mut() {
            self.run_function(func);
        }
    }

    fn run_function<F>(&mut self, function: &mut F)
        where F: lang::Function {
        
        for bb in function.basic_blocks_mut() {
            self.run_block(bb);
        }
    }

    fn run_block<B>(&mut self, block: &mut B)
        where B: lang::BasicBlock {
        panic!("the pass is not implemented");
    }
}

