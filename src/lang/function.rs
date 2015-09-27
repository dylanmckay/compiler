
use lang;
use std::{self,fmt};

/// A function.
pub trait Function : Sized + fmt::Display {
    type BasicBlock: lang::BasicBlock;
    type Type: lang::Type;
    
    fn signature<'a>(&'a self) -> &'a lang::Signature<Self::Type>;

    fn basic_blocks<'a>(&'a self) -> std::slice::Iter<'a,Self::BasicBlock>;
    fn basic_blocks_mut<'a>(&'a mut self) -> std::slice::IterMut<'a,Self::BasicBlock>;

    fn map<F,T>(self, _: F) -> T
        where F: Fn(Self::BasicBlock) -> T::BasicBlock,
              T: Function {
        unimplemented!();
    }
}
