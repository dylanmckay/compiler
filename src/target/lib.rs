pub use self::target::Target;

pub use self::machine::avr::{self, AVR};

pub mod target;
pub mod machine;

extern crate compiler_ir as ir;
extern crate compiler_mir as mir;
extern crate compiler_select as select;

