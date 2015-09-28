
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

    fn run_block(&mut self, block: &<M::Function as lang::Function>::BasicBlock) {
        use lang::BasicBlock;

        for inst in block.instructions() {
            self.run_instruction(inst);
        }
    }

    fn run_instruction(&mut self,
                       _: &<<M::Function as lang::Function>::BasicBlock as lang::BasicBlock>::Instruction) {

        panic!("the pass is not implemented");
    }
}

pub trait PassMut<M> : Metadata
    where M: lang::Module
{
    fn run_module(&mut self, module: M) -> M{
        module.map_functions(|a| self.run_function(a))
    }

    fn run_function(&mut self, function: M::Function)
        -> M::Function {
        use lang::Function;

        function.map_blocks(|a| self.run_block(a))
    }

    fn run_block(&mut self, block: <M::Function as lang::Function>::BasicBlock)
        -> <M::Function as lang::Function>::BasicBlock {
        use lang::BasicBlock;

        block.map_instructions(|a| self.run_instruction(a))
    }

    fn run_instruction(&mut self,
                       _: <<M::Function as lang::Function>::BasicBlock as lang::BasicBlock>::Instruction)
        -> <<M::Function as lang::Function>::BasicBlock as lang::BasicBlock>::Instruction {

        panic!("the pass is not implemented");
    }
}

