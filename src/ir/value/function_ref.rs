use {Function,Type,types,Signature};
use util;

// FIXME: remove 'name'
// Also clean up 'ir::read::resolve' afterwards

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct FunctionRef
{
    func_id: util::Id,

    name: String,
    ty: types::Function,
}

impl FunctionRef
{
    pub fn new(func_id: util::Id,
               name: String,
               ty: types::Function) -> Self {
        FunctionRef {
            func_id: func_id,
            name: name,
            ty: ty,
        }
    }

    pub fn reference(func: &Function) -> Self {
        FunctionRef {
            func_id: func.id(),
            name: func.name().into(),
            ty: types::Function::new(func.signature().clone()),
        }
    }

    /// Gets the name of the function.
    pub fn name(&self) -> &str { &self.name }

    /// Gets the signature of the callee.
    pub fn signature(&self) -> &Signature {
        self.ty.signature()
    }

    pub fn function_id(&self) -> util::Id {
        self.func_id
    }

    pub fn ty(&self) -> Type {
        Type::pointer(self.ty.clone().into())
    }
}

impl_expression!(FunctionRef);
