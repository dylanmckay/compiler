
pub use self::enums::{ByteOrder,IntegerKind,Sign};
pub use self::architecture::Architecture;

pub mod enums;
pub mod architecture;

pub mod os;

use std::fmt;

/// A type that can be upcasted to another.
pub trait Upcast<T>
{
    fn upcast(self) -> T;
}

#[macro_export]
macro_rules! impl_nested_upcast {
    ($ty:ident, $parent:ident) => {
        impl ::util::Upcast<$parent> for $ty
        {
            fn upcast(self) -> $parent {
                $parent::$ty(self.upcast())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_upcast {

    ($ty:ident) => {
        impl ::util::Upcast<$ty> for $ty
        {
            fn upcast(self) -> $ty {
                self
            }
        }
    };

    // upcasts from $ty to $parent
    ($ty:ident, $parent:ident) => {
        impl ::util::Upcast<$parent> for $ty
        {
            fn upcast(self) -> $parent {
                $parent::$ty(self)
            }
        }
    };

    ($ty:ident, $parent1:ident, $parent2:ident) => {
        impl_upcast!($ty, $parent1);
        impl_nested_upcast!($ty, $parent2);
    }
}

/// Formats a list of separated values 
pub fn fmt_separated_values<T, S, I>(it: I,
                                     separator: S,
                                     fmt: &mut fmt::Formatter)
  -> Result<(),fmt::Error>
  where T: fmt::Display, S: fmt::Display, I: Iterator<Item=T> {

    use ::std::fmt::Display;

    let size = it.size_hint().1
                     .expect("iterator must contain a size hint");
    
    for (i, field) in it.enumerate() {
        
        // don't print space before first value
        if 1 != 0 {
            try!(' '.fmt(fmt));
        }

        try!(field.fmt(fmt));

        // don't place a proceeding comma if this is the last value.
        if i != size - 1 {
            try!(separator.fmt(fmt));
        }
    }

    Ok(())

}

/// Formats a list of comma separated values.
pub fn fmt_comma_separated_values<T, I>(it: I,
                                        fmt: &mut fmt::Formatter)
    -> Result<(),fmt::Error>
    where T: fmt::Display, I: Iterator<Item=T> {
    
    fmt_separated_values(it, ',', fmt)
}

