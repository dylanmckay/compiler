use {Function,Global,Parameter,Module,Value,Expression,Type,Register,Block};
use util::{Id,Identifiable};

pub enum Info
{
    Function {
        ty: ::types::Function,
    },
    Global {
        ty: Type,
    },
    Argument {
        ty: Type,
    },
    Register {
        ty: Type,
    },
    Block,
}

/// Something that may be resolved.
pub trait Resolvable : Identifiable
{
    fn name(&self) -> String;
    fn info(&self) -> Info;
}

impl Resolvable for Function {
    fn name(&self) -> String { Function::name(self).to_owned() }
    fn info(&self) -> Info {
        Info::Function {
            ty: ::types::Function::new(self.signature().clone()),
        }
    }
}

impl Resolvable for Global {
    fn name(&self) -> String { Global::name(self).to_owned() }
    fn info(&self) -> Info {
        Info::Global {
            ty: self.ty()
        }
    }
}

impl Resolvable for Parameter {
    fn name(&self) -> String { Parameter::name(self).to_owned() }
    fn info(&self) -> Info {
        Info::Argument {
            ty: self.ty().clone(),
        }
    }
}

impl Resolvable for Register {
    fn name(&self) -> String {
        let n: ::Name = Register::name(self).clone();
        n.into()
    }

    fn info(&self) -> Info {
        Info::Register {
            ty: self.ty(),
        }
    }
}

impl Resolvable for Block {
    fn name(&self) -> String {
        Block::name(self).into()
    }
    fn info(&self) -> Info {
        Info::Block
    }
}

pub struct Resolve
{
    scope_stack: Vec<Scope>,
}

impl Resolve
{
    pub fn new() -> Self {
        let global_scope = Scope::new();

        Resolve {
            scope_stack: vec![global_scope],
        }
    }

    pub fn reference(&mut self, name: String) -> Expression {
        // Use the global context if the item
        // has not been resolved yet.
        for scope in self.scope_stack.iter_mut() {
            if scope.has_defined(&name) {
                return scope.reference(name);
            }
        }

        // if the item has not been defined yet, it must
        // lay in the global scope.
        self.global_scope_mut().reference(name)
    }

    pub fn give<T>(&mut self, item: &mut T)
        where T: Resolvable + 'static {
        self.local_scope_mut().give(item);
    }

    pub fn resolve(&mut self, module: Module)
        -> Module {
        self.scope_stack.iter_mut().fold(module, |m,scope| {
            scope.resolve(m)
        })
    }

    pub fn begin_scope(&mut self) {
        self.scope_stack.push(Scope::new());
    }

    pub fn end_scope(&mut self) {
        assert!(self.scope_stack.len() > 1,
                "cannot close the global scope");

        self.scope_stack.pop();
    }

    fn global_scope_mut(&mut self) -> &mut Scope {
        self.scope_stack.first_mut().unwrap()
    }

    fn local_scope_mut(&mut self) -> &mut Scope {
        self.scope_stack.last_mut().unwrap()
    }
}

struct Scope
{
    items: Vec<Item>,
}

impl Scope
{
    /// Creates a new resolver.
    pub fn new() -> Self {
        Scope {
            items: Vec::new(),
        }
    }

    pub fn has_defined(&mut self, name: &str) -> bool {
        self.lookup_name(&name).map_or(false, |item| item.is_resolved())
    }

    /// Gives back an `ID` which will be used to reference an item.
    pub fn reference(&mut self, name: String) -> Expression {
        if let Some(item) = self.lookup_name(&name) {
            if item.is_resolved() {
                return item.make_reference().unwrap();
            } else {
                return Expression::UnresolvedRef(item.id);
            }
        }

        let id = Id::next();

        self.items.push(Item::unresolved(id, name));

        Expression::UnresolvedRef(id)
    }

    pub fn give<T>(&mut self, item: &mut T)
        where T: Resolvable + 'static {
        if let Some(a) =  self.lookup_name_mut(&item.name()) {
            a.resolve(item);
            return;
        }

        // otherwise create a new item
        self.items.push(Item::resolved(item))
    }

    pub fn resolve(&mut self, module: Module)
        -> Module {
        module.map_values(|v| self.resolve_value(v))
    }

    fn resolve_value(&mut self, value: Value) -> Value {
        let a = value.node.map_subvalues(|v| self.resolve_value(v));

        Value {
            node: match a {
                Expression::UnresolvedRef(id) => self.resolve_reference(id),
                other => other,
            },
        }
    }

    fn resolve_reference(&self, id: Id) -> Expression {
        match self.maybe_resolve_reference(id) {
            Some(expr) => expr,
            None => Expression::UnresolvedRef(id),
        }
    }

    fn maybe_resolve_reference(&self, id: Id) -> Option<Expression> {
        self.find_id(id).make_reference()
    }

    fn find_id(&self, id: Id) -> &Item {
        self.items.iter().find(|a| a.id == id)
            .expect("no item with that ID was found")
    }

    fn lookup_name(&self, name: &str)
        -> Option<&Item> {
        self.items.iter().find(|a| a.name == name)
    }

    fn lookup_name_mut(&mut self, name: &str)
        -> Option<&mut Item> {
        self.items.iter_mut().find(|a| a.name == name)
    }
}

struct Item
{
    id: Id,
    name: String,
    info: Option<Info>,
}

impl Item
{
    pub fn unresolved(id: Id,
                      name: String) -> Self {
        Item {
            id: id,
            name: name,
            info: None,
        }
    }

    pub fn resolved<T>(item: &T) -> Self
        where T: Resolvable + 'static {
        Item {
            id: item.get_id(),
            name: item.name(),
            info: Some(item.info()),
        }
    }

    pub fn resolve<T>(&mut self, item: &mut T)
        where T: Resolvable + 'static {
        // Make the item have the ID we have have been referring to it as.
        item.internal_set_id(self.id);

        self.info = Some(item.info());
    }

    pub fn is_resolved(&self) -> bool { self.info.is_some() }

    pub fn make_reference(&self) -> Option<Expression> {
        use ::value::{FunctionRef,GlobalRef,ArgumentRef,RegisterRef,BlockRef};

        let info = if let Some(ref i) = self.info { i } else { return None; };

        match *info {
            Info::Function { ref ty } => {
                Some(FunctionRef::new(
                    self.id,
                    self.name.clone(),
                    ty.clone()
                ).into())
            },
            Info::Global { ref ty } => {
                Some(GlobalRef::new(
                    self.id,
                    ty.clone(),
                ).into())
            },
            Info::Argument { ref ty } => {
                Some(ArgumentRef::new(
                    self.id,
                    ty.clone(),
                ).into())
            },
            Info::Register { ref ty } => {
                Some(RegisterRef::new(
                    self.id,
                    ty.clone(),
                ).into())
            },
            Info::Block => {
                println!("ref");
                Some(BlockRef::new(self.id).into())
            }
        }
    }
}

