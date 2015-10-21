
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

impl ir::value::ValueTrait for RegisterRef { }

impl Into<ir::Value> for RegisterRef
{
    fn into(self) -> ir::Value {
        ir::Value::RegisterRef(self)
    }
}
