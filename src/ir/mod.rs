
pub use self::types::{Type,TypeTrait};
pub use self::value::{Value,Expression,ExpressionTrait};
pub use lang::Name;
pub use ir::instruction::{Instruction,InstructionTrait};
pub use self::users::Users;
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
/// IR reading routines.
pub mod read;
/// Expression user information.
pub mod users;

pub mod cond;

pub type Module = ::lang::Module<Value>;
pub type Global = ::lang::Global<Value>;
pub type Function = ::lang::Function<Value>;
pub type Block = ::lang::Block<Value>;
pub type Signature = ::lang::Signature<Value>;
pub type Parameter = ::lang::Parameter<Value>;

