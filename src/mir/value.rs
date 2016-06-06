use Type;

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
    ConstantInteger {
        bit_width: u32,
        value: i64,
    },
}

impl Value
{
    pub fn ty(&self) -> Type {
        match *self {
            Value::Local { ref ty, .. } => ty.clone(),
            Value::Argument { ref ty, .. } => ty.clone(),
            Value::ConstantInteger { bit_width, .. } => {
                Type::Integer { bit_width: bit_width }
            },
        }
    }

    pub fn i(width: u32, value: i64) -> Self {
        Value::ConstantInteger {
            bit_width: width,
            value: value,
        }
    }

    pub fn expect_constant_integer(&self) -> i64 {
        if let Value::ConstantInteger { value, .. } = *self {
            value
        } else {
            panic!("expected a constant integer");
        }
    }
}
