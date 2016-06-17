pub use self::idiotic::Idiotic;

pub mod idiotic;

use {Target, Program};

/// A register allocation algorithm.
pub trait Algorithm<T: Target>
{
    fn allocate(&mut self, program: Program<T>) -> Program<T>;
}

/// Creates the default register allocator.
pub fn default<T: Target>(target: &T) -> Idiotic<T>
{
    Idiotic::new(target)
}

