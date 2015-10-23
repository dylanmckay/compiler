
use ir::Value;

/// Keeps track of the users of a value.
#[derive(Clone,Debug)]
pub struct Users<'a>
{
    users: Vec<&'a Value>,
}

impl<'a> Users<'a>
{
    /// Creates a new user list.
    pub fn new<I>(users: I) -> Self
        where I: Iterator<Item=&'a Value> {

        Users {
            users: users.collect(),
        }
    }

    /// Gets the values that are users.
    pub fn users(&self) -> ::std::slice::Iter<&'a Value> {
        self.users.iter()
    }
}
