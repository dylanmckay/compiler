
pub use self::dce::DeadCodeElimination;

/// Passes which operate on IR.
pub mod ir;

/// The dead code elimination pass.
pub mod dce;
