use {Type,Register};
use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct RegisterRef
{
    pub register_id: util::Id,
    pub ty: Type,
}

impl RegisterRef
{
    pub fn new(register_id: util::Id,
               ty: Type) -> Self {
        RegisterRef {
            register_id: register_id,
            ty: ty,
        }
    }

    pub fn reference(register: &Register) -> Self {
        use util::Identifiable;

        RegisterRef {
            register_id: register.get_id(),
            ty: register.ty().clone(),
        }
    }

    pub fn register_id(&self) -> util::Id { self.register_id }

    pub fn ty(&self) -> Type {
        self.ty.clone()
    }
}

impl_expression!(RegisterRef);
