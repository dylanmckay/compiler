
use ir;

/// A branching condition.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Condition
{
    /// Always.
    True,
    /// Never.
    False,

    /// Equal to.
    Equal(Box<ir::Expression>, Box<ir::Expression>),
    NotEqual(Box<ir::Expression>, Box<ir::Expression>),

    GreaterThan(Box<ir::Expression>, Box<ir::Expression>),
    GreaterThanOrEq(Box<ir::Expression>, Box<ir::Expression>),
    LessThan(Box<ir::Expression>, Box<ir::Expression>),
    LessThanOrEq(Box<ir::Expression>, Box<ir::Expression>),
}

impl Condition
{
    /// Checks if the condition is `true` or `false`.
    pub fn is_trivial(&self) -> bool {
        match *self {
            Condition::True |
            Condition::False => true,
            _ => false,
        }
    }

    /// Gets the textual abbreviation of the code.
    pub fn abbreviation(&self) -> &'static str {
        match *self {
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

