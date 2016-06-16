use {Program, Instruction};

/// Allocates registers for a set of instructions.
pub fn allocate<I>(instructions: Vec<I>) -> Vec<I>
    where I: Instruction {
    Program::build(instructions.into_iter()).
        allocate().
        into_instructions()
}

