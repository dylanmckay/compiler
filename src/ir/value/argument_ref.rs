use {Type,Parameter};
use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct ArgumentRef
{
    id: util::Id,
    ty: Type,
}

impl ArgumentRef
{
    pub fn reference(parameter: &Parameter) -> Self {
        use util::Identifiable;

        ArgumentRef {
            id: parameter.get_id(),
            ty: parameter.ty().clone(),
        }
    }

    pub fn ty(&self) -> Type {
        self.ty.clone()
    }
}

impl_expression!(ArgumentRef);
