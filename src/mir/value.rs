use Type;
use ir;

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Value
{
    Local {
        number: u32,
        ty: Type,
    },
    Argument {
        number: u32,
        ty: Type,
    },
    Integer {
        bit_width: u16,
        value: i64,
    },
}

impl Value
{
    pub fn ty(&self) -> Type {
        match *self {
            Value::Local { ref ty, .. } => ty.clone(),
            Value::Argument { ref ty, .. } => ty.clone(),
            Value::Integer { bit_width, .. } => {
                Type::Integer { bit_width: bit_width }
            },
        }
    }
}
