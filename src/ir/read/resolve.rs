use {Function,Global};
use util::{Id,Identifiable};

/// Something that may be resolved.
pub trait Resolvable : Identifiable
{
    fn name(&self) -> &str;
}

impl Resolvable for Function {
    fn name(&self) -> &str { Function::name(self) }
}

impl Resolvable for Global {
    fn name(&self) -> &str { Global::name(self) }
}
pub struct Resolve<T: Resolvable>
{
    items: Vec<ResolveItem<T>>,
}

impl<T> Resolve<T>
    where T: Resolvable
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

    pub fn resolve(&mut self, item: T) {
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

    pub fn give_to<O>(self, object: &mut O)
        where O: Extend<T> {
        object.extend(self.into_items());
    }

    fn into_items(self) -> ::std::vec::IntoIter<T> {
        // TODO: remove unnecessay allocations
        self.items.into_iter()
                  .filter(ResolveItem::is_resolved)
                  .map(|item_resolve| item_resolve.item.unwrap())
                  .collect::<Vec<_>>()
                  .into_iter()
    }

    fn lookup_name(&self, name: &str)
        -> Option<&ResolveItem<T>> {
        self.items.iter().find(|a| a.name == name)
    }

    fn lookup_name_mut(&mut self, name: &str)
        -> Option<&mut ResolveItem<T>> {
        self.items.iter_mut().find(|a| a.name == name)
    }
}

struct ResolveItem<T: Resolvable>
{
    id: Id,
    name: String,
    item: Option<T>,
}

impl<T> ResolveItem<T>
    where T: Resolvable
{
    pub fn unresolved(id: Id,
                      name: String) -> Self {
        ResolveItem {
            id: id,
            name: name,
            item: None,
        }
    }

    pub fn resolved(id: Id,
                    name: String,
                    item: T) -> Self {
        ResolveItem {
            id: id,
            name: name,
            item: Some(item),
        }
    }

    pub fn is_resolved(&self) -> bool { self.item.is_some() }

    pub fn resolve(&mut self, mut item: T) {
        // Make the item have the ID we have have been referring to it as.
        item.internal_set_id(self.id);
        self.item = Some(item);
    }
}

