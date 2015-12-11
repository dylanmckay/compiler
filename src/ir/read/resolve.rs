use {Function,Global,Module,Value,Expression,Type};
use util::{Id,Identifiable};

pub enum ResolvableInfo
{
    Function {
        ty: ::types::Function,
    },
    Global {
        ty: Type,
    },
}

/// Something that may be resolved.
pub trait Resolvable : Identifiable
{
    fn name(&self) -> &str;
    fn give_to(self: Box<Self>, module: &mut Module);
    fn info(&self) -> ResolvableInfo;
}

impl Resolvable for Function {
    fn name(&self) -> &str { Function::name(self) }
    fn give_to(self: Box<Self>, module: &mut Module) {
        module.add_function(*self);
    }
    fn info(&self) -> ResolvableInfo {
        ResolvableInfo::Function {
            ty: ::types::Function::new(self.signature().clone()),
        }
    }
}

impl Resolvable for Global {
    fn name(&self) -> &str { Global::name(self) }
    fn give_to(self: Box<Self>, module: &mut Module) where Self: Sized {
        module.add_global(*self)
    }
    fn info(&self) -> ResolvableInfo {
        ResolvableInfo::Global {
            ty: self.ty()
        }
    }
}

pub struct Resolve
{
    items: Vec<ResolveItem>,
}

impl Resolve
{
    /// Creates a new resolver.
    pub fn new() -> Self {
        Resolve {
            items: Vec::new(),
        }
    }

    /// Gives back an `ID` which will be used to reference an item.
    pub fn reference(&mut self, name: String) -> Expression {
        if let Some(item) = self.lookup_name(&name) {
            return Expression::UnresolvedRef(item.id);
        }

        let id = Id::next();

        self.items.push(ResolveItem::unresolved(id, name));

        Expression::UnresolvedRef(id)
    }

    pub fn give<T>(&mut self, item: T)
        where T: Resolvable + 'static {
        if let Some(a) =  self.lookup_name_mut(item.name()) {
            a.resolve(item);
            return;
        }

        // otherwise create a new item
        self.items.push(ResolveItem::resolved(
            item.get_id(),
            item.name().to_owned(),
            item,
        ));
    }

    pub fn resolve(&mut self, mut module: Module)
        -> Module {
        // move the actual items (functions, globals)
        // into the module
        for item in self.items.iter_mut() {
            item.give_to(&mut module);
        }

        module.map_values(|v| self.resolve_value(v))
    }

    fn resolve_value(&mut self, value: Value) -> Value {
        let a = value.map_subvalues(|v| self.resolve_value(v));

        a.map_expression(|expr| match expr {
            Expression::UnresolvedRef(id) => self.resolve_reference(id),
            other => other,
        })
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

    fn find_id(&self, id: Id) -> &ResolveItem {
        self.items.iter().find(|a| a.id == id)
            .expect("no item with that ID was found")
    }

    fn lookup_name(&self, name: &str)
        -> Option<&ResolveItem> {
        self.items.iter().find(|a| a.name == name)
    }

    fn lookup_name_mut(&mut self, name: &str)
        -> Option<&mut ResolveItem> {
        self.items.iter_mut().find(|a| a.name == name)
    }
}

struct ResolveItem
{
    id: Id,
    name: String,
    item: Option<Box<Resolvable>>,
    info: Option<ResolvableInfo>,
}

impl ResolveItem
{
    pub fn unresolved(id: Id,
                      name: String) -> Self {
        ResolveItem {
            id: id,
            name: name,
            item: None,
            info: None,
        }
    }

    pub fn resolved<T>(id: Id,
                       name: String,
                       item: T) -> Self
        where T: Resolvable + 'static {
        ResolveItem {
            id: id,
            name: name,
            info: Some(item.info()),
            item: Some(Box::new(item)),
        }
    }

    pub fn give_to(&mut self, module: &mut Module) {
        use std::mem;

        let item = mem::replace(&mut self.item, None);

        if let Some(i) = item { i.give_to(module) }
    }

    pub fn resolve<T>(&mut self, mut item: T)
        where T: Resolvable + 'static {
        // Make the item have the ID we have have been referring to it as.
        item.internal_set_id(self.id);

        self.info = Some(item.info());
        self.item = Some(Box::new(item));
    }

    pub fn make_reference(&self) -> Option<Expression> {
        use ::value::{FunctionRef,GlobalRef};

        let info = if let Some(ref i) = self.info { i } else { return None; };

        match *info {
            ResolvableInfo::Function { ref ty } => {
                Some(FunctionRef::new(
                    self.id,
                    self.name.clone(),
                    ty.clone()
                ).into())
            },
            ResolvableInfo::Global { ref ty } => {
                Some(GlobalRef::new(
                    self.id,
                    ty.clone(),
                ).into())
            },
        }
    }
}

