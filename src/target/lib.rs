pub use self::target::Target;

pub use self::machine::avr;

pub mod target;
pub mod machine;

extern crate compiler_ir as ir;
