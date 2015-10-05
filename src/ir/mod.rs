
pub use self::types::{Type,TypeTrait};
pub use self::value::{Value,ValueTrait};
pub use self::name::Name;
pub use self::instruction::{Instruction,InstructionTrait};
pub use self::block::Block;
pub use self::function::Function;
pub use self::global::Global;
pub use self::module::Module;

pub mod types;
pub mod value;
pub mod name;

pub mod instruction;
pub mod block;
pub mod function;
pub mod global;
pub mod module;

/// The IR verifier.
pub mod verifier;
/// Routines for printing modules.
pub mod print;
