
pub use self::types::{Type,TypeTrait};
pub use self::value::{Value,ValueTrait};
pub use self::name::Name;
pub use self::instruction::{Instruction,InstructionTrait};
pub use self::basicblock::Block;
pub use self::function::Function;
pub use self::module::Module;

pub mod types;
pub mod value;
pub mod name;

pub mod instruction;
// TODO: rename to 'block'.
pub mod basicblock;
pub mod function;
pub mod module;

/// The IR verifier.
pub mod verifier;
