
use util;
use std;

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

    /// Gets an element from the set.
    pub fn get(&self, id: util::Id) -> &T {
        self.elements.iter().find(|&a| a.get_id() == id).unwrap()
    }

    /// Adds an element to the set.
    ///
    /// Returns the ID of the element.
    pub fn add(&mut self, mut element: T) -> util::Id {
        let id = util::Id::next();

        self.elements.push(element);
        id
    }

    /// Gets an iterator to the elements in the set.
    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a,T> {
        self.elements.iter()
    }

    /// Gets a mutable iterator to the elements in the set,
    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a,T> {
        self.elements.iter_mut()
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
