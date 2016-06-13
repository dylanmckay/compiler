use {Type,Parameter};
use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct ArgumentRef
{
    pub param_id: util::Id,
    pub ty: Type,
}

impl ArgumentRef
{
    pub fn new(param_id: util::Id,
               ty: Type) -> Self {
        ArgumentRef {
            param_id: param_id,
            ty: ty,
        }
    }

    pub fn reference(parameter: &Parameter) -> Self {
        use util::Identifiable;

        ArgumentRef {
            param_id: parameter.get_id(),
            ty: parameter.ty().clone(),
        }
    }

    pub fn parameter_id(&self) -> util::Id {
        self.param_id
    }

    pub fn ty(&self) -> Type {
        self.ty.clone()
    }
}

impl_expression!(ArgumentRef);
