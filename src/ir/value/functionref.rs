
use ir;
use util;
use std;

#[derive(Clone,Debug)]
pub struct FunctionRef
{
    id: util::Id,

    name: String,
    signature: ir::types::Function,
}

impl FunctionRef
{
    pub fn reference(func: &ir::Function) -> Self {
        FunctionRef {
            id: func.id(),
            name: func.name().into(),
            signature: func.signature().clone(),
        }
    }

    /// Gets the name of the function.
    pub fn name(&self) -> &str { &self.name }

    /// Gets the signature of the function.
    pub fn signature(&self) -> &ir::types::Function {
        &self.signature
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
