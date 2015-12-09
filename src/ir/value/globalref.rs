use {Global,Type};
use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct GlobalRef
{
    id: util::Id,
    ty: Type,
}

impl GlobalRef
{
    pub fn reference(global: &Global) -> Self {
        GlobalRef {
            id: global.id(),
            ty: global.ty().clone(),
        }
    }

    pub fn global_id(&self) -> util::Id {
        self.id
    }

    pub fn ty(&self) -> Type {
        Type::pointer(self.ty.clone())
    }
}

impl_expression!(GlobalRef);
