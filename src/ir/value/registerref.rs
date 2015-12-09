use {Type,Register};
use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct RegisterRef
{
    id: util::Id,
    ty: Type,
}

impl RegisterRef
{
    pub fn reference(register: &Register) -> Self {
        use util::Identifiable;

        RegisterRef {
            id: register.get_id(),
            ty: register.ty().clone(),
        }
    }

    pub fn ty(&self) -> Type {
        self.ty.clone()
    }
}

impl_expression!(RegisterRef);
