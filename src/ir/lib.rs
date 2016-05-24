#![feature(iter_arith)]

pub use self::types::{Type,TypeTrait};
pub use self::value::{Value,Expression,ExpressionTrait,Register};
pub use self::instruction::{Instruction,InstructionTrait,Unary,Binary};
pub use self::print::printable;
pub use self::users::Users;
pub use self::cond::Condition;

pub use self::module::Module;
pub use self::global::Global;
pub use self::function::{Function,Signature,Parameter};
pub use self::block::Block;
pub use self::name::Name;

pub use self::attrs::*;

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
/// Global variable stuff.
pub mod global;
/// Function stuff.
pub mod function;
/// Basic block handling.
pub mod block;
/// A name.
pub mod name;

/// Various attributes.
pub mod attrs;

extern crate compiler_util as util;

extern crate num;
extern crate bit_vec;

