extern crate num;
extern crate bit_vec;

/// Various utilities.
pub use self::compiler_util as util;
/// The intermediate representation.
pub use self::compiler_ir as ir;
/// The machine level IR.
pub use self::compiler_mir as mir;
/// The backend module.
pub use self::compiler_target as target;
/// The machine module.
pub use self::compiler_machine as machine;
/// The pass infrastructure.
pub use self::compiler_pass as pass;
/// The instruction selector.
pub use self::compiler_select as select;
/// The integrated tester.
pub use self::compiler_test as test;

pub extern crate compiler_util;
pub extern crate compiler_ir;
pub extern crate compiler_mir;
pub extern crate compiler_target;
pub extern crate compiler_machine;
pub extern crate compiler_pass;
pub extern crate compiler_select;
pub extern crate compiler_test;
