
pub mod instruction;
pub mod basicblock;
pub mod signature;
pub mod function;
pub mod module;
pub mod ty;

pub use self::instruction::Instruction;
pub use self::basicblock::BasicBlock;
pub use self::signature::Signature;
pub use self::function::Function;
pub use self::module::Module;
pub use self::ty::Type;
