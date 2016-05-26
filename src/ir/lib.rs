#![feature(iter_arith)]

pub use self::types::{Type,TypeTrait};
pub use self::value::{Value,Expression,ExpressionTrait,Register};
pub use self::instruction::{Instruction,InstructionTrait,Unary,Binary};
pub use self::print::printable;
pub use self::users::Users;
pub use self::cond::Condition;

pub use self::module::Module;

pub use self::item::Item;
pub use self::span::Spanned;
pub use self::item::Global;
pub use self::item::{Function,Signature,Parameter};

pub use self::block::Block;
pub use self::name::Name;

pub use self::attrs::*;

/// A item.
pub mod item;

pub mod span;

/// Where types are implemented.
pub mod types;
/// Contains the different value kinds.
pub mod value;
/// Where instruction definitions are located.
pub mod instruction;
/// The IR verifier.
pub mod verifier;
/// Routines for printing modules.
pub mod print;
/// IR reading routines.
pub mod read;
/// Expression user information.
pub mod users;
/// Condition codes.
pub mod cond;

/// Module stuff.
pub mod module;
/// Basic block handling.
pub mod block;
/// A name.
pub mod name;

/// Various attributes.
pub mod attrs;

extern crate compiler_util as util;

extern crate num;
extern crate bit_vec;

