
use std::{self,fmt};
use lang;

pub trait BasicBlock : fmt::Debug + fmt::Display
{
    type Instruction: lang::Instruction;

    fn instructions<'a>(&'a self) -> std::slice::Iter<'a,Self::Instruction>;
}
