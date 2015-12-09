use {Type,Expression,value};
use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct RegisterRef
{
    id: util::Id,
    ty: Type,
}

impl RegisterRef
{
    pub fn reference(register: &value::Register) -> Self {
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

impl util::Identifiable for RegisterRef
{
    fn get_id(&self) -> util::Id { self.id }
}

impl value::ExpressionTrait for RegisterRef { }

impl Into<Expression> for RegisterRef
{
    fn into(self) -> Expression {
        Expression::RegisterRef(self)
    }
}
