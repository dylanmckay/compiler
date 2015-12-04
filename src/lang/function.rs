
use lang;
use util;
use std;

use lang::Block;

/// A parameter.
#[derive(Clone,Debug)]
pub struct Parameter<V: lang::Value>
{
    id: util::Id,

    ty: V::Type,
    name: String,
}

impl<V> Parameter<V>
    where V: lang::Value
{
    pub fn new(name: String, ty: V::Type) -> Self {
        Parameter {
            id: util::Id::next(),

            name: name,
            ty: ty,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> &V::Type {
        &self.ty
    }
}

impl<V: lang::Value> util::Identifiable for Parameter<V>
{
    fn get_id(&self) -> util::Id { self.id }
}

impl<V> std::fmt::Display for Parameter<V>
    where V: lang::Value
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}: {}", self.name, self.ty)
    }
}

impl<V: lang::Value> std::cmp::PartialEq for Parameter<V>
    where V::Type: std::cmp::PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty &&
            self.name == other.name
    }
}

impl<V: lang::Value> std::cmp::Eq for Parameter<V>
    where V::Type: std::cmp::Eq {
}

/// A function signature.
/// 
/// Holds the return and parameter types.
#[derive(Clone,Debug)]
pub struct Signature<V: lang::Value>
{
    params: util::List<Parameter<V>>,
    return_types: Vec<V::Type>,
}

impl<V> Signature<V>
    where V: lang::Value
{
    pub fn new<P,R>(params: P, returns: R) -> Self
        where P: IntoIterator<Item=Parameter<V>>,
              R: IntoIterator<Item=V::Type> {
        Signature {
            params: params.into_iter().collect(),
            return_types: returns.into_iter().collect(),
        }
    }

    /// Creates a signature with no return types and no parameter types.
    pub fn empty() -> Self {
        Signature {
            params: util::List::empty(),
            return_types: Vec::new(),
        }
    }

    /// Appends a return type.
    pub fn ret(mut self, ty: V::Type) -> Self {
        self.return_types.push(ty);
        self
    }

    /// Appends a parameter type.
    pub fn param(mut self,
                 name: String,
                 ty: V::Type) -> Self {
        self.params.add(Parameter::new(name, ty));
        self
    }

    /// Gets the return types.
    pub fn returns(&self) -> std::slice::Iter<V::Type> {
        self.return_types.iter()
    }

    /// Gets the parameter types.
    pub fn parameters(&self) -> std::slice::Iter<Parameter<V>> {
        self.params.iter()
    }

    /// Looks up a parameter given its name.
    pub fn find_parameter(&self, name:&str) -> Option<&Parameter<V>> {
        self.params.iter().find(|param| param.name() == name)
    }
}

impl<V: lang::Value> std::cmp::PartialEq for Signature<V>
    where V::Type: std::cmp::PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.parameters().zip(other.parameters()).all(|(a,b)| a==b) &&
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
