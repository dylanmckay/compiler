use Type;

/// A register.
/// Stores the zero-based index of the node in the
/// DAG that is referred to.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Register {
    /// The number of the node that is referred to.
    /// Zero based.
    pub number: u32,
    /// The number of the result from the node.
    pub value_number: u32,
    /// The type.
    pub ty: Type,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Value
{
    /// An argument to the function.
    Argument {
        number: u32,
        ty: Type,
    },
    /// A constant integer.
    ConstantInteger {
        bit_width: u32,
        value: i64,
    },
    /// A register.
    Register(Register),
}

impl Value
{
    /// Gets the type of the value.
    pub fn ty(&self) -> Type {
        match *self {
            Value::Argument { ref ty, .. } => ty.clone(),
            Value::ConstantInteger { bit_width, .. } => {
                Type::Integer { bit_width: bit_width }
            },
            Value::Register(ref reg)  => reg.ty.clone(),
        }
    }

    /// Creates a new n-bit constant integer.
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
