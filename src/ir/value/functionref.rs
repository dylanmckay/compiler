
use ir;
use util;
use std;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct FunctionRef
{
    id: util::Id,

    name: String,
    ty: ir::types::Function,
}

impl FunctionRef
{
    pub fn reference(func: &ir::Function) -> Self {
        FunctionRef {
            id: func.id(),
            name: func.name().into(),
            ty: ir::types::Function::new(func.signature().clone()),
        }
    }

    /// Gets the name of the function.
    pub fn name(&self) -> &str { &self.name }

    /// Gets the signature of the callee.
    pub fn signature(&self) -> &ir::Signature {
        self.ty.signature()
    }

    pub fn ty(&self) -> ir::Type {
        ir::Type::pointer(self.ty.clone().into())
    }
}

impl std::fmt::Display for FunctionRef
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.id.fmt(fmt)
    }
}

impl ir::value::ValueTrait for FunctionRef { }

impl Into<ir::Value> for FunctionRef
{
    fn into(self) -> ir::Value {
        ir::Value::FunctionRef(self)
    }
}
