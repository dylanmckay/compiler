use Type;
use lang;
use ir;

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Value
{
    NotLowered(ir::Value),
}

impl lang::Value for Value
{
    type Type = Type;

    fn subvalues(&self) -> Vec<&Self> {
        match *self {
            Value::NotLowered(..) => vec![],
        }
    }

    fn map_subvalues<F>(self, f: F) -> Self
        where F: FnMut(Self) -> Self {
        unimplemented!();
    }

    fn ty(&self) -> Type {
        unimplemented!();
    }


    fn is_simple(&self) -> bool{ true }
    fn is_terminator(&self) -> bool {
        unimplemented!();
    }
}
