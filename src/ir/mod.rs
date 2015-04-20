
pub use self::types::{Type,TypeTrait};
pub use self::value::{Value,ValueTrait};
pub use self::name::Name;
pub use self::instructions::{Instruction,InstructionTrait};
pub use self::constants::{Constant};
pub use self::basicblock::BasicBlock;
pub use self::function::Function;

pub mod types;
pub mod value;
pub mod name;

pub mod instructions;
pub mod constants;
pub mod basicblock;
pub mod function;

