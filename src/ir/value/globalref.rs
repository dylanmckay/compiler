
use ir;
use util;
use std;

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

    pub fn ty(&self) -> ir::Type {
        ir::Type::pointer(self.ty.clone())
    }
}

impl std::fmt::Display for GlobalRef
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.id.fmt(fmt)
    }
}

impl ir::value::ValueTrait for GlobalRef { }

impl Into<ir::Value> for GlobalRef
{
    fn into(self) -> ir::Value {
        ir::Value::GlobalRef(self)
    }
}
