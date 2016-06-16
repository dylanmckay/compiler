pub use self::rdrr::*;
pub use self::rdi::*;
pub use self::simple::*;

pub use self::ldi::LDIRdK;
pub use self::mov::MOVRdRr;

#[macro_export]
macro_rules! impl_debug_for_instruction {
    ($name:ident) => {
        impl std::fmt::Debug for $name {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                try!(write!(fmt, "{} ", self.mnemonic()));

                let operands: Vec<_> = self.operands().iter().map(|op| format!("{:?}", op)).collect();
                try!(write!(fmt, "{}", operands.join(", ")));

                Ok(())
            }
        }
    }
}

// Instruction families.
pub mod rdrr;
pub mod rdi;
pub mod simple;

// Individual instructions.
pub mod ldi;
pub mod mov;

