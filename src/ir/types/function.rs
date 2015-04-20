
use ir::types::{Type,TypeTrait};
use lang;
use util;
use std::fmt;

/// Specifies the type of a function.
#[derive(Clone,Eq,PartialEq,Debug)]
pub struct Function
{
    pub signature: lang::function::Signature<Type>,
}

impl Function
{
    pub fn new(signature: lang::function::Signature<Type>) -> Function {
        Function {
            signature: signature,
        }
    }
}

impl TypeTrait for Function
{
    fn size(&self) -> u64 { 0 }

    fn upcast(self) -> Type {
        Type::Function(self)
    }
}

impl fmt::Display for Function
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        unimplemented!();
    }
}

impl lang::Type for Function { }
