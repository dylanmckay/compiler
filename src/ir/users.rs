
use ir;


/// Stores the users of a value.
pub struct UserInformation<'a>
{
    users: Vec<&'a ir::Value>,
}

impl<'a> UserInformation<'a>
{
    pub fn empty() -> Self {
        UserInformation {
            users: Vec::new(),
        }
    }

    pub fn users(&self) -> ::std::slice::Iter<&ir::Value> {
        self.users.iter()
    }
}
