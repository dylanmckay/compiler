pub use self::global::Global;
pub use self::function::{Function,Signature,Parameter};

pub mod global;
pub mod function;

pub trait ItemTrait : ::util::Identifiable + Into<Item>
{
}

#[derive(Debug,Clone)]
pub enum Item
{
    Global(Global),
    Function(Function),
}

