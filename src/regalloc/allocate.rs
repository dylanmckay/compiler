use {Target, Program, Algorithm};
use algorithm;

/// Allocates registers for a set of instructions.
pub fn allocate<T>(target: &T, instructions: Vec<T::Instruction>) -> Vec<T::Instruction>
    where T: Target {
    let mut algo = algorithm::default(target);

    let program = algo.allocate(Program::build(instructions.into_iter()));
    program.into_instructions()
}

