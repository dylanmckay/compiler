
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

impl<T> Set<T>
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
    pub fn add(&mut self, element: T) -> util::Id {
        let index = self.generator.next();

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

