use std;

pub trait Instruction : Clone + std::fmt::Debug
{
    type Slot: Slot;

    fn slots_mut(&mut self) -> Vec<Box<Slot>>;
}

pub trait Slot : std::fmt::Debug
{
    fn is_virtual(&self) -> bool;
}

