use {Type,Register};
use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct RegisterRef
{
    reg_id: util::Id,
    ty: Type,
}

impl RegisterRef
{
    pub fn reference(register: &Register) -> Self {
        use util::Identifiable;

        RegisterRef {
            reg_id: register.get_id(),
            ty: register.ty().clone(),
        }
    }

    pub fn register_id(&self) -> util::Id { self.reg_id }

    pub fn ty(&self) -> Type {
        self.ty.clone()
    }
}

impl_expression!(RegisterRef);
