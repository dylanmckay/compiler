
use std::{self,fmt};
use lang;

pub trait BasicBlock : fmt::Debug + fmt::Display
{
    type Instruction: lang::Instruction;

    fn instructions<'a>(&'a self) -> std::slice::Iter<'a,Self::Instruction>;
    fn instructions_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, Self::Instruction>;

    fn map_instructions<F>(self, f: F) -> Self
        where F: FnMut(Self::Instruction) -> Self::Instruction;

    /// Replaces the instruction list from an iterator.
    fn with_instructions<I>(self, instructions: I) -> Self
        where I: Iterator<Item=Self::Instruction>;
}
