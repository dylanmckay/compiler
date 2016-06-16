pub use self::rdrr::*;
pub use self::rdi::*;
pub use self::simple::*;

pub use self::ldi::LDIRdK;
pub use self::mov::MOVRdRr;

// Instruction families.
pub mod rdrr;
pub mod rdi;
pub mod simple;

// Individual instructions.
pub mod ldi;
pub mod mov;
