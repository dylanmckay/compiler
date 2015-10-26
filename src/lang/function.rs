
use lang;
use util;
use std;

use lang::Block;

/// A function signature.
/// 
/// Holds the return and parameter types.
#[derive(Clone,Debug)]
pub struct Signature<V: lang::Value>
{
    param_types: Vec<V::Type>,
    return_types: Vec<V::Type>,
}

impl<V> Signature<V>
    where V: lang::Value
{
    /// Creates a signature with no return types and no parameter types.
    pub fn empty() -> Self {
        Signature {
            param_types: Vec::new(),
            return_types: Vec::new(),
        }
    }

    /// Appends a return type.
    pub fn ret(mut self, ty: V::Type) -> Self {
        self.return_types.push(ty);
        self
    }

    /// Appends a parameter type.
    pub fn param(mut self, ty: V::Type) -> Self {
        self.param_types.push(ty);
        self
    }

    /// Gets the return types.
    pub fn returns(&self) -> std::slice::Iter<V::Type> {
        self.return_types.iter()
    }

    /// Gets the parameter types.
    pub fn parameters(&self) -> std::slice::Iter<V::Type> {
        self.param_types.iter()
    }
}

impl<V: lang::Value> std::cmp::PartialEq for Signature<V>
    where V::Type: std::cmp::PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.param_types == other.param_types &&
        self.return_types == other.return_types
    }
}

impl<V: lang::Value> std::cmp::Eq for Signature<V>
    where V::Type: std::cmp::Eq
{
}

/// A function.
#[derive(Clone,Debug)]
pub struct Function<V: lang::Value>
{
    id: util::Id,

    name: String,
    signature: Signature<V>,
    blocks: Vec<Block<V>>,

    cc: lang::CallingConvention,

    inline_hint: lang::InlineHint,
    complexity_hint: lang::ComplexityHint,
}

impl<V> Function<V>
    where V: lang::Value
{
    /// Creates a new function.
    pub fn new<N>(name: N,
                  signature: Signature<V>,
                  blocks: Vec<Block<V>>) -> Self
        where N: Into<String> {

        Function {
            id: util::Id::next(),

            name: name.into(),
            signature: signature,
            blocks: blocks,

            cc: lang::CallingConvention::default(),
            inline_hint: lang::InlineHint::default(),
            complexity_hint: lang::ComplexityHint::default(),
        }
    }

    /// Creates an empty function.
    pub fn empty<N>(name: N, sig: Signature<V>) -> Self
        where N: Into<String> {
        Function::new(name, sig, Vec::new())
    }

    /// Appends a block to the function.
    pub fn append_block(&mut self, block: Block<V>) {
        self.blocks.push(block);
    }

    /// Gets the name of the function.
    pub fn name(&self) -> &str { &self.name }

    /// Gets the signature of the function.
    pub fn signature(&self) -> &Signature<V> {
        &self.signature
    }

    /// Flattens the values in the function.
    pub fn flatten(self) -> Self {
        self.map_blocks(|b| b.flatten())
    }

    /// Gets the ID of the function.
    pub fn id(&self) -> util::Id { self.id }

    /// Gets the blocks that the function contains.
    pub fn blocks(&self) -> std::slice::Iter<Block<V>> {
        self.blocks.iter()
    }

    /// Gets a mutable iterator to the contained blocks.
    pub fn blocks_mut(&mut self) -> std::slice::IterMut<Block<V>> {
        self.blocks.iter_mut()
    }

    /// Performs a mapping over the blocks of the function.
    pub fn map_blocks<F>(mut self, f: F) -> Self
        where F: FnMut(Block<V>) -> Block<V> {

        let blocks = self.blocks.into_iter().map(f);
        self.blocks = blocks.collect();

        self
    }

    /// Sets the blocks that the function contains.
    pub fn with_blocks<I>(mut self, blocks: I) -> Self
        where I: Iterator<Item=Block<V>> {

        self.blocks = blocks.collect();
        self
    }

    /// Gets the values that the function contains.
    pub fn values(&self) -> std::vec::IntoIter<&V> {
        // FIXME: return 'impl Iterator' once supported
        let vals: Vec<_> = self.blocks.iter().flat_map(Block::values).collect();
        vals.into_iter()
    }

    /// Gets the calling convention.
    pub fn calling_convention(&self) -> lang::CallingConvention {
        self.cc
    }

    /// Gets the inline hint.
    pub fn inline_hint(&self) -> lang::InlineHint {
        self.inline_hint
    }

    /// Gets the complexity hint.
    pub fn complexity_hint(&self) -> lang::ComplexityHint {
        self.complexity_hint
    }
}

impl<V: lang::Value> util::Identifiable for Function<V>
{
    fn get_id(&self) -> util::Id { self.id }
}

impl<V: PartialEq + lang::Value> std::cmp::PartialEq for Function<V>
{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.signature == other.signature
    }
}

impl<V: Eq + lang::Value> std::cmp::Eq for Function<V> { }
