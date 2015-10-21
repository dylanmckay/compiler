
use ir;
use util;
use std;

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

impl std::fmt::Display for RegisterRef
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.id.fmt(fmt)
    }
}

impl ir::value::ValueTrait for RegisterRef { }

impl Into<ir::Value> for RegisterRef
{
    fn into(self) -> ir::Value {
        ir::Value::RegisterRef(self)
    }
}
