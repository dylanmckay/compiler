
use ir;
use util;
use std;

#[derive(Clone,Debug)]
pub struct FunctionRef
{
    id: util::Id,
    signature: ir::types::Function,
}

impl FunctionRef
{
    pub fn reference(func: &ir::Function) -> Self {
        FunctionRef {
            id: func.id(),
            signature: func.signature().clone(),
        }
    }
}

impl std::fmt::Display for FunctionRef
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.id.fmt(fmt)
    }
}

impl ir::value::ValueTrait for FunctionRef
{
    fn ty(&self) -> ir::Type {
        self.signature.clone().into()
    }
}

impl Into<ir::Value> for FunctionRef
{
    fn into(self) -> ir::Value {
        ir::Value::FunctionRef(self)
    }
}
