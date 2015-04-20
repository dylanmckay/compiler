
pub mod manager;

pub use self::manager::Manager;

use lang;

/// A pass identifier.
pub type Id = u64;

/// A pass over a set of instructions.
pub trait Pass
{
    fn id(&self) -> Id;
    
    /// Gets the identifiers of the passes this pass
    /// depends on.
    fn dependencies(&self) -> &'static [Id] {
        &[]
    }
}

pub trait ModulePass : Pass
{
    type Module: lang::Module;
}

pub trait ModulePassMut : Pass
{
    type Module: lang::Module;
}

