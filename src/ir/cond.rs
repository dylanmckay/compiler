use Value;

/// A branching condition.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Condition
{
    /// Always.
    True,
    /// Never.
    False,

    /// Equal to.
    Equal(Box<Value>, Box<Value>),
    NotEqual(Box<Value>, Box<Value>),

    GreaterThan(Box<Value>, Box<Value>),
    GreaterThanOrEq(Box<Value>, Box<Value>),
    LessThan(Box<Value>, Box<Value>),
    LessThanOrEq(Box<Value>, Box<Value>),
}

impl Condition
{
    pub fn from_boolean(value: bool) -> Self {
        if value { Condition::True } else { Condition::False }
    }

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

