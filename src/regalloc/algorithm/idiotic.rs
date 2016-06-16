use {Algorithm, Program, Instruction};

/// An idiotic register allocator.
///
/// Everything is stored on the stack until it is needed.
pub struct Idiotic;

impl Algorithm for Idiotic
{
    fn allocate<I>(&mut self, program: Program<I>) -> Program<I>
        where I: Instruction {
        program
    }
}

