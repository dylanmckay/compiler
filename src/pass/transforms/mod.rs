
pub use self::constant_folding::ConstantFolding;
pub use self::strength_reduction::StrengthReduction;
pub use self::dce::DeadCodeElimination;

pub mod constant_folding;
pub mod strength_reduction;
pub mod dce;
