extern crate num;
extern crate bit_vec;

/// The intermediate representation.
pub use self::compiler_ir as ir;
/// The machine module.
pub use self::compiler_machine as machine;
/// The machine level IR.
pub use self::compiler_mir as mir;
/// The pass infrastructure.
pub use self::compiler_pass as pass;
/// The register allocator.
pub use self::compiler_regalloc as regalloc;
/// The instruction selector.
pub use self::compiler_select as select;
/// The backend.
pub use self::compiler_target as target;
/// The integrated tester.
pub use self::compiler_test as test;
/// Various utilities.
pub use self::compiler_util as util;

pub extern crate compiler_ir;
pub extern crate compiler_machine;
pub extern crate compiler_mir;
pub extern crate compiler_pass;
pub extern crate compiler_regalloc;
pub extern crate compiler_select;
pub extern crate compiler_target;
pub extern crate compiler_test;
pub extern crate compiler_util;

