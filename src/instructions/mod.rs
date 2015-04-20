
pub use self::instruction::{Instruction,InstructionTrait};

pub use self::add::Add;
pub use self::ret::Return;

pub mod instruction;

pub mod add;
pub mod ret;
