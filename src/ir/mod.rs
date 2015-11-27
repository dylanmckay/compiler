
pub use self::types::{Type,TypeTrait};
pub use self::value::{Expression,ExpressionTrait};
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
/// Expression user information.
pub mod users;

pub mod cond;

pub type Module = ::lang::Module<Expression>;
pub type Global = ::lang::Global<Expression>;
pub type Function = ::lang::Function<Expression>;
pub type Block = ::lang::Block<Expression>;
pub type Signature = ::lang::Signature<Expression>;
