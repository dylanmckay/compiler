use {Target, Algorithm, Program};

/// An idiotic register allocator.
///
/// Everything is stored on the stack until it is needed.
pub struct Idiotic;

impl Algorithm for Idiotic
{
    fn allocate<T>(&mut self, _target: &T, program: Program<T>) -> Program<T>
        where T: Target {
        program
    }
}

