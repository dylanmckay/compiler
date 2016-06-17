use {Target, Algorithm, Program, Instruction, Operand, RegisterClass};
use std::collections::HashMap;

use util;

/// An idiotic register allocator.
pub struct Idiotic<'a, T: Target+'a>
{
    pub target: &'a T,
}

impl<'a, T: Target> Idiotic<'a, T>
{
    pub fn new(target: &'a T) -> Self {
        Idiotic {
            target: target,
        }
    }
}

impl<'a, T: Target> Algorithm<T> for Idiotic<'a, T>
{
    fn allocate(&mut self, mut program: Program<T>) -> Program<T>
        where T: Target {
        
        let mut register_map: HashMap<util::Id, T::Register> = HashMap::new();

        program.items = program.items.into_iter().map(|mut item| {
            item.instruction = self::allocate_instruction::<T>(item.instruction, &mut register_map);
            item
        }).collect();
        program
    }
}

fn allocate_instruction<T: Target>(mut instruction: T::Instruction,
                                   register_map: &mut HashMap<util::Id, T::Register>) -> T::Instruction {
    for operand in instruction.operands_mut() {
        if operand.is_virtual() {
            let register_class = operand.register_class();
            let register = register_class.registers()[0].clone();
            operand.allocate(register);
        }
    }

    instruction
}

