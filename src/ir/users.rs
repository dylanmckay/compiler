use Expression;

/// Stores the users of a value.
pub struct Users<'a>
{
    users: Vec<&'a Expression>,
}

impl<'a> Users<'a>
{
    pub fn empty() -> Self {
        Users {
            users: Vec::new(),
        }
    }

    pub fn users(&self) -> ::std::slice::Iter<&Expression> {
        self.users.iter()
    }
}
