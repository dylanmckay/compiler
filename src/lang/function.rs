
use lang;
use std::{self,fmt};

/// A function.
pub trait Function : Sized + fmt::Display {
    type Block: lang::Block;
    type Type: lang::Type;
    
    fn signature<'a>(&'a self) -> &'a lang::Signature<Self::Type>;

    fn blocks<'a>(&'a self) -> std::slice::Iter<'a,Self::Block>;
    fn blocks_mut<'a>(&'a mut self) -> std::slice::IterMut<'a,Self::Block>;

    fn map_blocks<F>(self, f: F) -> Self
        where F: FnMut(Self::Block) -> Self::Block;

    fn with_blocks<I>(self, blocks: I) -> Self
        where I: Iterator<Item=Self::Block>;

    fn map<F,T>(self, _: F) -> T
        where F: Fn(Self::Block) -> T::Block,
              T: Function {
        unimplemented!();
    }
}
