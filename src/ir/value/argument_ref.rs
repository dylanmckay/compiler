use ir;
use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct ArgumentRef
{
    id: util::Id,
    ty: ir::Type,
}

impl ArgumentRef
{
    pub fn reference(parameter: &ir::Parameter) -> Self {
        use util::Identifiable;

        ArgumentRef {
            id: parameter.get_id(),
            ty: parameter.ty().clone(),
        }
    }

    pub fn ty(&self) -> ir::Type {
        self.ty.clone()
    }
}

impl util::Identifiable for ArgumentRef
{
    fn get_id(&self) -> util::Id { self.id }
}

impl ir::value::ExpressionTrait for ArgumentRef { }

impl Into<ir::Expression> for ArgumentRef
{
    fn into(self) -> ir::Expression {
        ir::Expression::ArgumentRef(self)
    }
}

