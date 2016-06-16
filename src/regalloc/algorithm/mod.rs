pub use self::idiotic::Idiotic;

pub mod idiotic;

use {Program, Instruction};

/// A register allocation algorithm.
pub trait Algorithm
{
    fn allocate<I>(&mut self, program: Program<I>) -> Program<I>
        where I: Instruction;
}

/// Creates the default register allocator.
pub fn default() -> Idiotic { Idiotic }

