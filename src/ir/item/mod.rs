pub use self::global::Global;
pub use self::function::{Function,Signature,Parameter};

pub mod global;
pub mod function;

use util;

#[derive(Clone,Debug)]
pub enum Item
{
    Global(Global),
    Function(Function),
}

impl util::Identifiable for Item
{
    fn get_id(&self) -> util::Id {
        match *self {
            Item::Global(ref i) => i.get_id(),
            Item::Function(ref i) => i.get_id(),
        }
    }

    fn internal_set_id(&mut self, id: util::Id) {
        match self {
            &mut Item::Global(ref mut i) => i.internal_set_id(id),
            &mut Item::Function(ref mut i) => i.internal_set_id(id),
        }
    }
}

