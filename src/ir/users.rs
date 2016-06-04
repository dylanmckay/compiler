use {Value, Expression, Item, Module, Function, Global, Block};
use util::Identifiable;

/// Stores the users of a value.
pub struct Users<'a>
{
    users: Vec<&'a Value>,
}

impl<'a> Users<'a>
{
    pub fn empty() -> Self {
        Users {
            users: Vec::new(),
        }
    }

    /// Gets all of the users of an item in a module.
    pub fn of(item: &Item, module: &'a Module) -> Self {
        let mut users = Vec::new();

        for f in module.functions() {
            users_in_function(item, f, &mut users);
        }

        for g in module.globals() {
            users_in_global(item, g, &mut users);
        }

        Users { users: users }
    }

    pub fn len(&self) -> usize {
        self.users.len()
    }

    pub fn users(&self) -> ::std::slice::Iter<&Value> {
        self.users.iter()
    }
}

fn users_in_global<'a>(item: &Item,
                       g: &'a Global,
                       users: &mut Vec<&'a Value>) {
    users_in_value(item, &g.value, users);
}

fn users_in_function<'a>(item: &Item,
                         f: &'a Function,
                         users: &mut Vec<&'a Value>) {
    for block in f.blocks() {
        users_in_block(item, block, users);
    }
}

fn users_in_block<'a>(item: &Item,
                      block: &'a Block,
                      users: &mut Vec<&'a Value>) {
    for value in block.values() {
        users_in_value(item, value, users);
    }
}

fn users_in_value<'a>(item: &Item,
                      value: &'a Value,
                      users: &mut Vec<&'a Value>) {
    match value.node {
        Expression::GlobalRef(ref global_ref) => if item.get_id() == global_ref.global_id {
            users.push(value);
        },
        Expression::FunctionRef(ref func_ref) => if item.get_id() == func_ref.func_id {
            users.push(value);
        },
        _ => {
            for subvalue in value.node.subvalues() {
                users_in_value(item, subvalue, users);
            }
        },
    }
}

