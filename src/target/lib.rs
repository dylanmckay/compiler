pub use self::registry::register;
pub use self::error::Error;

#[macro_use]
extern crate lazy_static;

pub mod registry;
pub mod error;

use std::io;

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum OutputType
{
    /// Textual assembly.
    Assembly,
}

/// A target.
pub trait Target : Sync
{
    fn name(&self) -> &'static str;
    fn display_name(&self) -> &'static str;

    fn output_types(&self) -> &'static [OutputType];

    fn generate(&self,
                output_type: OutputType,
                input: &mut io::Read,
                output: &mut io::Write)
        -> Result<(), Error>;
}

