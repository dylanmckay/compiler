use Expression;
use Item;
use Module;
use Block;

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

    pub fn of(item: &Item, module: &Module) -> Self {
        unimplemented!();
    }

    pub fn users(&self) -> ::std::slice::Iter<&Expression> {
        self.users.iter()
    }
}

fn users_in_block<'a>(item: &Item,
                      block: &'a Block,
                      users: &mut Vec<&'a Expression>) {

}

