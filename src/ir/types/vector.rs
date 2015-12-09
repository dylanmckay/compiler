use types::{Type,TypeTrait};
use std::fmt;

/// A vector value.
#[derive(Clone,Eq,PartialEq,Debug)]
pub struct Vector
{
    count: u64,
    ty: Box<Type>,
}

impl Vector
{
    /// Creates a new vector.
    pub fn new(count: u64, ty: Type) -> Vector {
        Vector {
            count: count,
            ty: Box::new(ty),
        }
    }
}

impl fmt::Display for Vector
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        try!('<'.fmt(fmt));
        try!(self.count.fmt(fmt));
        try!(" x ".fmt(fmt));
        try!(self.ty.fmt(fmt));
        '>'.fmt(fmt)
    }
}

impl TypeTrait for Vector
{
    fn size(&self) -> u64 {
        self.ty.size() as u64 * self.count
    }
}

impl_type!(Vector);
