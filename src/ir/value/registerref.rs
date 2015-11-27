
use ir;
use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct RegisterRef
{
    id: util::Id,
    ty: ir::Type,
}

impl RegisterRef
{
    pub fn reference(register: &ir::value::Register) -> Self {
        use util::Identifiable;

        RegisterRef {
            id: register.get_id(),
            ty: register.ty().clone(),
        }
    }

    pub fn ty(&self) -> ir::Type {
        self.ty.clone()
    }
}

impl util::Identifiable for RegisterRef
{
    fn get_id(&self) -> util::Id { self.id }
}

impl ir::value::ExpressionTrait for RegisterRef { }

impl Into<ir::Expression> for RegisterRef
{
    fn into(self) -> ir::Expression {
        ir::Expression::RegisterRef(self)
    }
}
