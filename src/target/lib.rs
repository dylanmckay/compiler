pub use self::registry::register;

#[macro_use]
extern crate lazy_static;

pub mod registry;

/// A target.
pub trait Target : Sync
{
    fn name(&self) -> &'static str;
    fn display_name(&self) -> &'static str;
}

