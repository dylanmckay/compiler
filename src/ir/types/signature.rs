
use ir::types::{Type,TypeTrait};
use lang;

/// A function signature in IR.
pub type Signature = lang::Signature<Type>;

impl TypeTrait for Signature
{
    fn size(&self) -> u64 {
        panic!("this should not be called");
    }

    fn upcast(self) -> Type {
        Type::Signature(self)
    }
}

impl_into_type!(Signature);
