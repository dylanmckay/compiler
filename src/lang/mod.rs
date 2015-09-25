
pub mod instruction;
pub mod basicblock;
pub mod function;
pub mod module;
pub mod ty;

pub use self::instruction::Instruction;
pub use self::basicblock::BasicBlock;
pub use self::function::{Signature,Function};
pub use self::module::Module;
pub use self::ty::Type;
