use {Global,Type};
use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct GlobalRef
{
    pub global_id: util::Id,
    pub ty: Type,
}

impl GlobalRef
{
    pub fn new(global_id: util::Id,
               ty: Type) -> Self {
        GlobalRef {
            global_id: global_id,
            ty: ty,
        }
    }

    pub fn reference(global: &Global) -> Self {
        GlobalRef {
            global_id: global.id(),
            ty: global.ty().clone(),
        }
    }

    pub fn global_id(&self) -> util::Id {
        self.global_id
    }

    pub fn ty(&self) -> Type {
        Type::pointer(self.ty.clone())
    }
}

impl_expression!(GlobalRef);
