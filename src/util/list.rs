
use {Id,Identifiable};
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
pub struct List<T: Identifiable>
{
    elements: Vec<T>,
    locked_indices: Vec<usize>,
}

impl<T: Identifiable> List<T>
{
    /// Creates an empty set.
    pub fn empty() -> Self {
        List {
            elements: Vec::new(),
            locked_indices: Vec::new(),
        }
    }

    pub fn lookup(&self, id: Id) -> Slot<&T> {
        self.elements.iter()
                     .enumerate()
                     .filter(|&(index, _)| !self.is_index_locked(index))
                     .map(|(_,a)| a)
                     .find(|&a| a.get_id() == id)
                     .into()
    }

    /// Gets an element from the set.
    pub fn get(&self, id: Id) -> &T {
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

    /// Checks if there are elements in the list.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    // TODO: Clone should be unnecessay
    //       It should be possible to replace the locked element with
    //       garbage data and move the value out.
    pub fn map_in_place<F>(&mut self, mut f: F)
        where F: FnMut(T) -> T, T: Clone {

        for index in 0..self.elements.len() {
            self.lock_index(index);
            {
                let copy = self.elements[index].clone();
                self.elements[index] = f(copy);
            }
            self.unlock_index(index);
        }
    }

    fn lock_index(&mut self, index: usize) {
        self.locked_indices.push(index);
    }

    fn unlock_index(&mut self, index: usize) {
        let i = self.locked_indices.iter()
                                   .position(|&i| i==index)
                                   .expect("the index does not exist");
        self.locked_indices.remove(i);
    }

    fn is_index_locked(&self, index: usize) -> bool {
        self.locked_indices.contains(&index)
    }
}

impl<T: Identifiable> IntoIterator for List<T>
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> std::vec::IntoIter<T> {
        self.elements.into_iter()
    }
}

impl<T: Identifiable> std::iter::FromIterator<T> for List<T>
{
    fn from_iter<I>(it: I) -> Self
        where I: IntoIterator<Item=T> {
        List {
            elements: Vec::from_iter(it),
            locked_indices: Vec::new(),
        }
    }
}

impl<T: Identifiable + std::fmt::Debug> std::fmt::Debug for List<T>
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let elements: Vec<_> = self.iter().collect();
        std::fmt::Debug::fmt(&elements, fmt)
    }
}

