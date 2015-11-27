
use ir;
use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct GlobalRef
{
    id: util::Id,
    ty: ir::Type,
}

impl GlobalRef
{
    pub fn reference(global: &ir::Global) -> Self {
        GlobalRef {
            id: global.id(),
            ty: global.ty().clone(),
        }
    }

    pub fn global_id(&self) -> util::Id {
        self.id
    }

    pub fn ty(&self) -> ir::Type {
        ir::Type::pointer(self.ty.clone())
    }
}

impl ir::value::ExpressionTrait for GlobalRef { }

impl Into<ir::Expression> for GlobalRef
{
    fn into(self) -> ir::Expression {
        ir::Expression::GlobalRef(self)
    }
}
