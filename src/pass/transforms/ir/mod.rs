

pub use self::constant_folding::ConstantFolding;
pub use self::strength_reduction::StrengthReduction;

/// The constant folder.
pub mod constant_folding;
/// The strength reduction pass.
pub mod strength_reduction;

