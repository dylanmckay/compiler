
use util;
use std;

/// A set.
// TODO: find a better name.
#[derive(Clone)]
pub struct Set<T>
{
    elements: Vec<T>,
    generator: util::id::Generator,
}

impl<T: util::id::Identifiable> Set<T>
{
    /// Creates an empty set.
    pub fn empty() -> Self {
        Set {
            elements: Vec::new(),
            generator: util::id::Generator::new(),
        }
    }

    /// Gets an element from the set.
    pub fn get(&self, id: util::Id) -> &T {
        let index = id.underlying();
        &self.elements[index]
    }

    /// Adds an element to the set.
    ///
    /// Returns the ID of the element.
    pub fn add(&mut self, mut element: T) -> util::Id {
        let index = self.generator.next();
        element.set_id(index);

        // we should always be appending to the end.
        debug_assert!(index.underlying() == self.elements.len());

        self.elements.push(element);
        index
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

impl<T> IntoIterator for Set<T>
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> std::vec::IntoIter<T> {
        self.elements.into_iter()
    }
}

impl<T> std::iter::FromIterator<T> for Set<T>
{
    fn from_iter<I>(it: I) -> Self
        where I: IntoIterator<Item=T> {
        Set {
            elements: Vec::from_iter(it),
            generator: util::id::Generator::new(),
        }
    }
}

impl<T: std::fmt::Debug + util::id::Identifiable> std::fmt::Debug for Set<T>
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let elements: Vec<_> = self.iter().collect();
        std::fmt::Debug::fmt(&elements, fmt)
    }
}
