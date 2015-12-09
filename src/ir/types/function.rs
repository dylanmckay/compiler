use types::TypeTrait;
use Signature;
use std;

/// A function signature in IR.
#[derive(Clone,Eq,PartialEq,Debug)]
pub struct Function
{
    signature: Signature,
}

impl Function
{
    pub fn new(signature: Signature) -> Self {
        Function {
            signature: signature,
        }
    }

    /// Gets the function signature.
    pub fn signature(&self) -> &Signature {
        &self.signature
    }
}

impl std::fmt::Display for Function
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use util;

        write!(fmt, "({}) ({})",
               util::comma_separated_values(self.signature.returns()),
               util::comma_separated_values(self.signature.parameters()))
    }
}

impl TypeTrait for Function { }

impl_type!(Function);
