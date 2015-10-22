
pub use self::types::{Type,TypeTrait};
pub use self::value::{Value,ValueTrait};
pub use lang::Name;
pub use ir::instruction::{Instruction,InstructionTrait};
pub use self::cond::Condition;

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

pub mod cond;

pub type Module = ::lang::Module<Value>;
pub type Global = ::lang::Global<Value>;
pub type Function = ::lang::Function<Value>;
pub type Block = ::lang::Block<Value>;
pub type Signature = ::lang::Signature<Value>;
