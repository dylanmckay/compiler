use {Program, Instruction, Algorithm};
use algorithm;

/// Allocates registers for a set of instructions.
pub fn allocate<I>(instructions: Vec<I>) -> Vec<I>
    where I: Instruction {
    let mut algo = algorithm::default();

    let program = algo.allocate(Program::build(instructions.into_iter()));
    program.into_instructions()
}

