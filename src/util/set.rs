
use util;
use std;

/// When mapping over a set.
pub enum Slot<T>
{
    Here(T),
    Current,
}

impl<T> Slot<T>
{
    pub fn expect<S>(self, msg: S) -> T
        where S: Into<String> {
        match self {
            Slot::Here(a) => a,
            Slot::Current => panic!(msg.into()),
        }
    }
}

impl<T> From<Option<T>> for Slot<T>
{
    fn from(option: Option<T>) -> Slot<T> {
        match option {
            Some(thing) => Slot::Here(thing),
            None => Slot::Current,
        }
    }
}

impl<T> Into<Option<T>> for Slot<T>
{
    fn into(self) -> Option<T> {
        match self {
            Slot::Here(t) => Some(t),
            Slot::Current => None,
        }
    }
}

/// A set.
// TODO: find a better name.
#[derive(Clone)]
pub struct Set<T: util::Identifiable>
{
    elements: Vec<T>,
}

impl<T: util::Identifiable> Set<T>
{
    /// Creates an empty set.
    pub fn empty() -> Self {
        Set {
            elements: Vec::new(),
        }
    }

    pub fn lookup(&self, id: util::Id) -> Slot<&T> {
        self.elements.iter().find(|&a| a.get_id() == id).into()
    }

    /// Gets an element from the set.
    pub fn get(&self, id: util::Id) -> &T {
        self.lookup(id).expect("no element with that ID was found")
    }

    /// Adds an element to the set.
    ///
    /// Returns the ID of the element.
    pub fn add(&mut self, element: T)  {
        self.elements.push(element);
    }

    /// Gets an iterator to the elements in the set.
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.elements.iter()
    }

    /// Gets a mutable iterator to the elements in the set,
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.elements.iter_mut()
    }

    // TODO: Clone should be unnecessay
    pub fn map_in_place<F>(&mut self, mut f: F)
        where F: FnMut(T) -> T, T: Clone {
        for elem in self.elements.iter_mut() {
            let copy = elem.clone();
            *elem = f(copy);
        }
    }

}

impl<T: util::Identifiable> IntoIterator for Set<T>
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> std::vec::IntoIter<T> {
        self.elements.into_iter()
    }
}

impl<T: util::Identifiable> std::iter::FromIterator<T> for Set<T>
{
    fn from_iter<I>(it: I) -> Self
        where I: IntoIterator<Item=T> {
        Set {
            elements: Vec::from_iter(it),
        }
    }
}

impl<T: util::Identifiable + std::fmt::Debug> std::fmt::Debug for Set<T>
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let elements: Vec<_> = self.iter().collect();
        std::fmt::Debug::fmt(&elements, fmt)
    }
}
