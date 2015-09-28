
use std::fmt;
use lang;

pub trait BasicBlock : Sized + fmt::Debug + fmt::Display
{
    type Value: lang::Value;

    fn subvalues(&self) -> Vec<Self::Value>;

    fn with_subvalues<I>(self, values: I) -> Self
        where I: Iterator<Item=Self::Value>;

    /// Maps values to other values.
    fn map_subvalues<F>(self, f: F) -> Self
        where F: FnMut(Self::Value) -> Self::Value;

    fn map<F,T>(self, f: F) -> T
        where F: Fn(Self) -> T {
        f(self)
    }

}
