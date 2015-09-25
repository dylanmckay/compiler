
use std::{self,fmt};
use lang;

pub trait BasicBlock : fmt::Debug + fmt::Display
{
    type Instruction: lang::Instruction;

    fn instructions<'a>(&'a self) -> std::slice::Iter<'a,Self::Instruction>;
    fn instructions_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, Self::Instruction>;
}
