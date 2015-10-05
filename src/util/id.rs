
use std;

/// A unique identifier.
#[derive(Copy,Clone,Debug)]
pub struct Id(usize);

/// A special value signifying an unspeficied Id.
const UNSPECIFIED_ID: usize = !0;

impl Id
{
    /// Gets an ID which represents 
    pub fn unspecified() -> Self {
        Id(UNSPECIFIED_ID)
    }

    /// Checks if the ID is unspecified.
    pub fn is_specified(self) -> bool {
        let Id(val) = self;
        val != UNSPECIFIED_ID
    }

    /// Gets the underlying ID.
    pub fn underlying(self) -> usize {
        let Id(val) = self;
        val
    }
}

/// An object which has an identifier.
pub trait Identifiable
{
    /// Sets the internal ID of the object.
    /// This **should not** be called manually.
    fn set_id(&mut self, id: Id);
}

/// A unique identifier generator.
#[derive(Copy,Clone,Debug)]
pub struct Generator
{
    next: usize,
}

impl Generator
{
    /// Creates a new generator.
    pub fn new() -> Self {
        Generator {
            next: 0,
        }
    }

    /// Gets a new identifier.
    pub fn next(&mut self) -> Id {

        if self.next == UNSPECIFIED_ID {
            panic!("the ID space has been exhausted");
        }

        let id = self.next;
        self.next += 1;

        Id(id)
    }
}

impl std::cmp::PartialEq for Id
{
    fn eq(&self, other: &Id) -> bool {

        assert!(self.is_specified() && other.is_specified(),
                "id's have not been assigned");

        let &Id(i1) = self;
        let &Id(i2) = other;

        i1 == i2
    }
}

impl std::cmp::Eq for Id { }

impl std::fmt::Display for Id
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let &Id(val) = self;
        val.fmt(fmt)
    }
}
