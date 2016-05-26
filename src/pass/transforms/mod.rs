
pub use self::dce::DeadCodeElimination;
pub use self::constant_folding::ConstantFolding;
pub use self::strength_reduction::StrengthReduction;
pub use self::inliner::Inliner;

/// Implements a test that checks that a set of values
/// is mapped to another set of values.
///
/// It is of the form
/// ```
/// value_mapping_test!(test_name : mapping_fn }
///     input_value => output_value,
/// });
/// ```
macro_rules! value_mapping_test {
    (
        $name:ident : $mapper:path {
            $( $input:expr => $output:expr ),*
        }
    ) => {
        #[test]
        #[allow(unused_imports)]
        fn $name() {
            use ir::{self,Instruction,Value,Expression,Type};

            let cases = [
                $( ($input, $output) ),*
            ];

            for &(ref input, ref expected) in cases.iter() {
                let mapped = $mapper(input.clone().into());

                assert_eq!(mapped, expected.clone().into());
            }
        }
    }
}

/// The dead code elimination pass.
pub mod dce;
/// The constant folder.
pub mod constant_folding;
/// The strength reduction pass.
pub mod strength_reduction;
/// The inliner.
pub mod inliner;
