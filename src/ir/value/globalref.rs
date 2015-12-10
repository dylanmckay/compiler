use {Global,Type};
use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct GlobalRef
{
    global_id: util::Id,
    ty: Type,
}

impl GlobalRef
{
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
