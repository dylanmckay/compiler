use {Item,ItemTrait,Value,Type,Block};
use {InlineHint,ComplexityHint,CallingConvention};

use util;
use std;

/// A parameter.
#[derive(Clone,Debug)]
pub struct Parameter
{
    id: util::Id,

    ty: Type,
    name: String,
}

impl Parameter
{
    pub fn new(name: String, ty: Type) -> Self {
        Parameter {
            id: util::Id::next(),

            name: name,
            ty: ty,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }
}

impl util::Identifiable for Parameter
{
    fn get_id(&self) -> util::Id { self.id }
    fn internal_set_id(&mut self, id: util::Id) {
        self.id = id;
    }
}

impl std::fmt::Display for Parameter
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}: {}", self.name, self.ty)
    }
}

impl std::cmp::PartialEq for Parameter
{
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty &&
            self.name == other.name
    }
}

impl std::cmp::Eq for Parameter { }

/// A function signature.
/// 
/// Holds the return and parameter types.
#[derive(Clone,Debug)]
pub struct Signature
{
    params: util::List<Parameter>,
    return_types: Vec<Type>,
}

impl Signature
{
    pub fn new<P,R>(params: P, returns: R) -> Self
        where P: IntoIterator<Item=Parameter>,
              R: IntoIterator<Item=Type> {
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
    pub fn ret(mut self, ty: Type) -> Self {
        self.return_types.push(ty);
        self
    }

    /// Appends a parameter type.
    pub fn param(mut self,
                 name: String,
                 ty: Type) -> Self {
        self.params.add(Parameter::new(name, ty));
        self
    }

    /// Gets the return types.
    pub fn returns(&self) -> std::slice::Iter<Type> {
        self.return_types.iter()
    }

    /// Checks if the function has return values.
    pub fn has_returns(&self) -> bool {
        !self.return_types.is_empty()
    }

    /// Gets the parameter types.
    pub fn parameters(&self) -> std::slice::Iter<Parameter> {
        self.params.iter()
    }

    /// Checks if the function has any parameters.
    pub fn has_parameters(&self) -> bool {
        !self.params.is_empty()
    }

    /// Looks up a parameter given its name.
    pub fn find_parameter(&self, name:&str) -> Option<&Parameter> {
        self.params.iter().find(|param| param.name() == name)
    }

    /// Looks up a parameter given its ID.
    pub fn find_parameter_by_id(&self, id: util::Id)
        -> Option<&Parameter> {
        use util::Identifiable;
        self.params.iter().find(|param| param.get_id() == id)
    }
}

impl std::cmp::PartialEq for Signature
{
    fn eq(&self, other: &Self) -> bool {
        self.parameters().zip(other.parameters()).all(|(a,b)| a==b) &&
        self.return_types == other.return_types
    }
}

impl std::cmp::Eq for Signature { }

/// A function.
#[derive(Clone,Debug)]
pub struct Function
{
    id: util::Id,

    name: String,
    signature: Signature,
    blocks: Vec<Block>,

    cc: CallingConvention,

    inline_hint: InlineHint,
    complexity_hint: ComplexityHint,
}

impl Function
{
    /// Creates a new function.
    pub fn new<N>(name: N,
                  signature: Signature,
                  blocks: Vec<Block>) -> Self
        where N: Into<String> {

        Function {
            id: util::Id::next(),

            name: name.into(),
            signature: signature,
            blocks: blocks,

            cc: CallingConvention::default(),
            inline_hint: InlineHint::default(),
            complexity_hint: ComplexityHint::default(),
        }
    }

    /// Creates an empty function.
    pub fn empty<N>(name: N, sig: Signature) -> Self
        where N: Into<String> {
        Function::new(name, sig, Vec::new())
    }

    /// Appends a block to the function.
    pub fn append_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    /// Gets the name of the function.
    pub fn name(&self) -> &str { &self.name }

    /// Gets the signature of the function.
    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    /// Flattens the values in the function.
    pub fn flatten(self) -> Self {
        self.map_blocks(|b| b.flatten())
    }

    /// Gets the ID of the function.
    pub fn id(&self) -> util::Id { self.id }

    /// Gets the blocks that the function contains.
    pub fn blocks(&self) -> std::slice::Iter<Block> {
        self.blocks.iter()
    }

    /// Gets a mutable iterator to the contained blocks.
    pub fn blocks_mut(&mut self) -> std::slice::IterMut<Block> {
        self.blocks.iter_mut()
    }

    /// Performs a mapping over the blocks of the function.
    pub fn map_blocks<F>(mut self, f: F) -> Self
        where F: FnMut(Block) -> Block {

        let blocks = self.blocks.into_iter().map(f);
        self.blocks = blocks.collect();

        self
    }

    /// Sets the blocks that the function contains.
    pub fn with_blocks<I>(mut self, blocks: I) -> Self
        where I: Iterator<Item=Block> {

        self.blocks = blocks.collect();
        self
    }

    /// Gets the values that the function contains.
    pub fn values(&self) -> std::vec::IntoIter<&Value> {
        // FIXME: return 'impl Iterator' once supported
        let vals: Vec<_> = self.blocks.iter().flat_map(Block::values).collect();
        vals.into_iter()
    }

    pub fn map_values<F>(mut self, mut f: F) -> Self
        where F: FnMut(Value) -> Value {
        self.blocks = self.blocks.into_iter()
                                 .map(|b| b.map_values(|a| f(a)))
                                 .collect();
        self
    }

    /// Gets the calling convention.
    pub fn calling_convention(&self) -> CallingConvention {
        self.cc
    }

    /// Gets the inline hint.
    pub fn inline_hint(&self) -> InlineHint {
        self.inline_hint
    }

    /// Gets the complexity hint.
    pub fn complexity_hint(&self) -> ComplexityHint {
        self.complexity_hint
    }
}

impl util::Identifiable for Function
{
    fn get_id(&self) -> util::Id { self.id }
    fn internal_set_id(&mut self, id: util::Id) { self.id = id; }
}

impl std::cmp::PartialEq for Function
{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.signature == other.signature
    }
}

impl ItemTrait for Function { }

impl std::cmp::Eq for Function { }

impl Into<Item> for Function
{
    fn into(self) -> Item { Item::Function(self) }
}

