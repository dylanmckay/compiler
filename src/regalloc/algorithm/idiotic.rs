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

fn next_free_register<T: Target>(class: &T::RegisterClass, register_map: &HashMap<util::Id, T::Register>)
    -> T::Register {
    let used_registers: Vec<_> = register_map.values().filter(|&register| {
        class.contains(register.clone())
    }).collect();

    class.registers().into_iter().
        find(|register| !used_registers.iter().any(|r| r == &register)).
        expect("ran out of registers")
}

fn allocate_instruction<T: Target>(mut instruction: T::Instruction,
                                   register_map: &mut HashMap<util::Id, T::Register>) -> T::Instruction {
    for operand in instruction.operands_mut() {
        if operand.is_virtual() {
            let register_class = operand.register_class();
            let virt_id = operand.virtual_register_id();

            let next_free_register = self::next_free_register::<T>(&register_class, register_map);
            let register = register_map.entry(virt_id).or_insert(next_free_register).clone();

            operand.allocate(register);
        }
    }

    instruction
}

