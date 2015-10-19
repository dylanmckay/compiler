
use ir;
use std::fmt;

/// A branching condition.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Condition
{
    /// Always.
    True,
    /// Never.
    False,

    /// Equal to.
    Equal(Box<ir::Value>, Box<ir::Value>),
    NotEqual(Box<ir::Value>, Box<ir::Value>),

    GreaterThan(Box<ir::Value>, Box<ir::Value>),
    GreaterThanOrEq(Box<ir::Value>, Box<ir::Value>),
    LessThan(Box<ir::Value>, Box<ir::Value>),
    LessThanOrEq(Box<ir::Value>, Box<ir::Value>),
}

impl Condition
{
    /// Checks if the condition is `true` or `false`.
    pub fn is_trivial(self) -> bool {
        match self {
            Condition::True |
            Condition::False => true,
            _ => false,
        }
    }

    /// Gets the textual abbreviation of the code.
    pub fn abbreviation(self) -> &'static str {
        match self {
            Condition::True => "true",
            Condition::False => "false",
            Condition::Equal(..) => "eq",
            Condition::NotEqual(..) => "neq",
            Condition::GreaterThan(..) => "gt",
            Condition::GreaterThanOrEq(..) => "gte",
            Condition::LessThan(..) => "lt",
            Condition::LessThanOrEq(..) => "lte",
        }
    }
}

impl fmt::Display for Condition
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Condition::True => "true".fmt(fmt),
            &Condition::False => "false".fmt(fmt),
            &Condition::Equal(ref a, ref b) => write!(fmt, "{} eq {}", a, b),
            &Condition::NotEqual(ref a, ref b) => write!(fmt, "{} neq {}", a, b),
            &Condition::GreaterThan(ref a, ref b) => write!(fmt, "{} gt {}", a, b),
            &Condition::GreaterThanOrEq(ref a, ref b) => write!(fmt, "{} gte {}", a, b),
            &Condition::LessThan(ref a, ref b) => write!(fmt, "{} lt {}", a, b),
            &Condition::LessThanOrEq(ref a, ref b) => write!(fmt, "{} lte {}", a, b),
        }
    }
}

