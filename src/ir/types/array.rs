
use ir::types::{Type,TypeTrait};
use std::fmt;

/// Represents an array of values of a single `Type`.
#[derive(Clone,Eq,PartialEq,Debug)]
pub struct Array
{
    count: u64,
    ty: Box<Type>,
}

impl Array
{
    pub fn empty(ty: Type) -> Self {
        Array::new(0, ty)
    }

    pub fn new(count: u64, ty: Type) -> Array {
        Array {
            count: count,
            ty: Box::new(ty),
        }
    }
}

impl TypeTrait for Array
{
    fn size(&self) -> u64 {
        self.ty.size() * self.count
    }
}

impl fmt::Display for Array
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        try!('['.fmt(fmt));
        try!(self.count.fmt(fmt));
        try!(" x ".fmt(fmt));
        try!(self.ty.fmt(fmt));
        ']'.fmt(fmt)
    }
}

impl_type!(Array);
