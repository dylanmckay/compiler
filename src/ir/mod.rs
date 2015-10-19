
pub use self::types::{Type,TypeTrait};
pub use self::value::{Value,ValueTrait};
pub use lang::Name;
pub use ir::instruction::{Instruction,InstructionTrait};
pub use self::cond::Condition;

pub mod types;
pub mod value;
pub mod cond;

pub mod instruction;

/// The IR verifier.
pub mod verifier;
/// Routines for printing modules.
pub mod print;

use lang;
pub type Module = lang::Module<Value>;
pub type Global = lang::Global<Value>;
pub type Function = lang::Function<Value>;
pub type Block = lang::Block<Value>;
pub type Signature = lang::Signature<Value>;
