
use ir;
use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct FunctionRef
{
    func_id: util::Id,

    name: String,
    ty: ir::types::Function,
}

impl FunctionRef
{
    pub fn reference(func: &ir::Function) -> Self {
        FunctionRef {
            func_id: func.id(),
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

    pub fn function_id(&self) -> util::Id {
        self.func_id
    }

    pub fn ty(&self) -> ir::Type {
        ir::Type::pointer(self.ty.clone().into())
    }
}

impl ir::value::ExpressionTrait for FunctionRef { }

impl Into<ir::Expression> for FunctionRef
{
    fn into(self) -> ir::Expression {
        ir::Expression::FunctionRef(self)
    }
}
