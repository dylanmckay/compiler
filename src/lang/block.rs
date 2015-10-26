
use lang;
use util;

/// A basic block is a list of instructions which
/// end with a single terminator instruction.
#[derive(Clone,Debug)]
pub struct Block<V>
{
    id: util::Id,

    name: String,
    body: Vec<V>,
}

impl<V> Block<V>
    where V: lang::Value
{
    /// Creates a new basic block.
    pub fn new<N>(name: N,
                  body: Vec<V>) -> Self
        where N: Into<String> {
        Block {
            id: util::Id::next(),
            name: name.into(),
            body: body,
        }
    }

    /// Creates a basic block with no contained instructions.
    pub fn empty<N>(name: N) -> Self
        where N: Into<String> {
        Block::new(name, Vec::new())
    }

    /// Appends a value to the basic block.
    pub fn append_value<T>(&mut self, value: T)
        where T: Into<V> {
        self.body.push(value.into());
    }

    /// Gets the name of the block.
    pub fn name(&self) -> &str { &self.name }

    /// Flattens the values in the block.
    pub fn flatten(self) -> Self {
        let mut block = Block {
            id: self.id.clone(),
            name: self.name.clone(),
            body: Vec::new(),
        };

        for value in self.body {
            let new_value = value.flatten(&mut block);
            block.append_value(new_value);
        }

        block
    }

    /// Gets the last instruction in the block.
    /// Panics if the block is empty, or the last value it is not a terminator.
    pub fn terminator(&self) -> &V {
        let last = self.body.last().expect("the basic block is empty");

        assert!(last.is_terminator(), "the basic block is not terminated");
        last
    }

    /// Gets the ID of the block.
    pub fn id(&self) -> util::Id { self.id }

    /// Gets the values that the block contains.
    pub fn values(&self) -> ::std::slice::Iter<V> {
        self.body.iter()
    }

    /// Gets the values that the block contains as mutable.
    pub fn values_mut(&mut self) -> values::ValuesMut<V> {
        values::ValuesMut::new(self)
    }

    /// Sets the values that the block contains.
    pub fn with_values<I>(mut self, values: I) -> Self
        where I: Iterator<Item=V> {

        self.body = values.collect();
        self
    }

    /// Performs a mapping on the values that the block contains.
    pub fn map_values<F>(mut self, f: F) -> Self
        where F: FnMut(V) -> V {
        self.body = self.body.into_iter().map(f).collect();
        self
    }

    /// Filters values out of the basic block.
    pub fn filter<F>(mut self, mut f: F) -> Self
        where F: FnMut(&V) -> bool {
        self.body = self.body.into_iter()
                             .filter(|a| f(a))
                             .collect();
        self
    }
}

pub mod values
{
    use super::Block;
    use lang;

    pub struct ValuesMut<'a, V: lang::Value+'a>
    {
        block: &'a mut Block<V>,
        cur_idx: usize,
    }

    impl<'a,V> ValuesMut<'a,V>
        where V: lang::Value
    {
        pub fn new(block: &'a mut Block<V>) -> Self {
            ValuesMut {
                block: block,
                cur_idx: 0,
            }
        }

        pub fn insert_before(&mut self, value: V) {
            self.block.body.insert(self.cur_idx, value);
            self.cur_idx += 1;
        }

        pub fn insert_after(&mut self, value: V) {
            self.block.body.insert(self.cur_idx+1, value);
        }
    }

    impl<'a,V> Iterator for ValuesMut<'a,V>
        where V: lang::Value
    {
        type Item = &'a mut V;

        fn next(&mut self) -> Option<&'a mut V> {
            let next: Option<&mut V> = self.block.body.get_mut(self.cur_idx);

            if next.is_some() {
                self.cur_idx += 1;
            }
            // transmute the lifetime.
            unsafe {::std::mem::transmute(next) }
        }

        fn size_hint(&self) -> (usize,Option<usize>) {
            let len = self.block.body.len();
            (len, Some(len))
        }
    }
}
