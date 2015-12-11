use {Function,Global,Module};
use util::{Id,Identifiable};

/// Something that may be resolved.
pub trait Resolvable : Identifiable
{
    fn name(&self) -> &str;
    fn give_to(self: Box<Self>, module: &mut Module);
}

impl Resolvable for Function {
    fn name(&self) -> &str { Function::name(self) }
    fn give_to(self: Box<Self>, module: &mut Module) {
        module.add_function(*self);
    }
}

impl Resolvable for Global {
    fn name(&self) -> &str { Global::name(self) }
    fn give_to(self: Box<Self>, module: &mut Module) where Self: Sized {
        module.add_global(*self)
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
    pub fn reference(&mut self, name: String) -> Id {
        if let Some(item) = self.lookup_name(&name) {
            return item.id;
        }

        let id = Id::next();

        self.items.push(ResolveItem::unresolved(id, name));

        id
    }

    pub fn resolve<T>(&mut self, item: T)
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

    pub fn give_to(self, module: &mut Module) {
        for resolve_item in self.items.into_iter() {
            if let Some(item) = resolve_item.item {
                item.give_to(module);
            }
        }
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
}

impl ResolveItem
{
    pub fn unresolved(id: Id,
                      name: String) -> Self {
        ResolveItem {
            id: id,
            name: name,
            item: None,
        }
    }

    pub fn resolved<T>(id: Id,
                       name: String,
                       item: T) -> Self
        where T: Resolvable + 'static {
        ResolveItem {
            id: id,
            name: name,
            item: Some(Box::new(item)),
        }
    }

    pub fn is_resolved(&self) -> bool { self.item.is_some() }

    pub fn resolve<T>(&mut self, mut item: T)
        where T: Resolvable + 'static {
        // Make the item have the ID we have have been referring to it as.
        item.internal_set_id(self.id);
        self.item = Some(Box::new(item));
    }
}

