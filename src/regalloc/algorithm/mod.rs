pub use self::idiotic::Idiotic;

pub mod idiotic;

use {Target, Program};

/// A register allocation algorithm.
pub trait Algorithm
{
    fn allocate<T>(&mut self, target: &T, program: Program<T>) -> Program<T>
        where T: Target;
}

/// Creates the default register allocator.
pub fn default() -> Idiotic { Idiotic }

