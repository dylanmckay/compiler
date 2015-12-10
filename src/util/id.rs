
use std;

static mut ID_ACCUMULATOR: u64 = 0;

/// A unique identifier.
#[derive(Copy,Clone,Debug,Hash)]
pub struct Id(u64);


impl Id
{
    // FIXME: fix this race condition
    pub fn next() -> Self {
        let id = unsafe {
            Id(ID_ACCUMULATOR)
        };

        unsafe {
            ID_ACCUMULATOR += 1;
        }
        id
    }

    /// Gets the underlying ID.
    pub fn underlying(self) -> u64 {
        let Id(val) = self;
        val
    }
}

pub trait Identifiable
{
    fn get_id(&self) -> Id;

    fn internal_set_id(&mut self, id: Id);
}

impl std::cmp::PartialEq for Id
{
    fn eq(&self, other: &Id) -> bool {

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

