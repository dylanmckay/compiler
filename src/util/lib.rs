pub use self::enums::{ByteOrder,IntegerKind,Sign};
pub use self::architecture::Architecture;
pub use self::id::{Id,Identifiable};
pub use self::list::{List,Slot};
pub use self::set::Set;

pub mod enums;
pub mod architecture;
pub mod id;
pub mod list;
pub mod set;

pub mod os;

use std::fmt;

pub struct SeparatedValues<T,S>
{
    values: Vec<T>,
    separator: S,
}

/// Formats a list of separated values
pub fn separated_values<T,S,I>(it: I,
                               separator: S)
    -> SeparatedValues<T,S>
    where T: fmt::Display, S: fmt::Display, I: Iterator<Item=T> {

    SeparatedValues {
        values: it.collect(),
        separator: separator,
    }
}

impl<T,S> fmt::Display for SeparatedValues<T,S>
    where T: fmt::Display, S: fmt::Display
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {

        for (i, field) in self.values.iter().enumerate() {

            // don't print space before first value
            if i != 0 {
                try!(' '.fmt(fmt));
            }

            try!(field.fmt(fmt));

            // don't place a proceeding comma if this is the last value.
            if i != self.values.len() - 1 {
                try!(self.separator.fmt(fmt));
            }
        }
        Ok(())
    }
}

/// Formats a list of comma separated values.
pub fn comma_separated_values<T,I>(it: I)
    -> SeparatedValues<T,char>
    where T: fmt::Display, I: Iterator<Item=T> {

    separated_values(it, ',')
}

