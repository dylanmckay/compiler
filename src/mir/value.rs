use Type;

use util;
use std;

/// A register.
/// Stores the zero-based index of the node in the
/// DAG that is referred to.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct RegisterRef {
    /// The number of the node that is referred to.
    /// Zero based.
    pub register_id: util::Id,
    /// The number of the result from the node.
    pub result_number: u32,
    /// The type.
    pub ty: Type,
}

#[derive(Clone,PartialEq,Eq)]
pub enum Value
{
    /// An argument to the function.
    ArgumentRef {
        id: util::Id,
        ty: Type,
    },
    /// A constant integer.
    ConstantInteger {
        bit_width: u32,
        value: i64,
    },
    /// A register.
    RegisterRef(RegisterRef),
}

impl Value
{
    /// Gets the type of the value.
    pub fn ty(&self) -> Type {
        match *self {
            Value::ArgumentRef { ref ty, .. } => ty.clone(),
            Value::ConstantInteger { bit_width, .. } => {
                Type::Integer { bit_width: bit_width }
            },
            Value::RegisterRef(ref reg)  => reg.ty.clone(),
        }
    }

    /// Creates a new n-bit constant integer.
    pub fn i(width: u32, value: i64) -> Self {
        Value::ConstantInteger {
            bit_width: width,
            value: value,
        }
    }

    /// Creates a new register reference.
    pub fn register_ref(register_id: util::Id, result_number: u32, ty: Type) -> Self {
        Value::RegisterRef(RegisterRef {
            register_id: register_id,
            result_number: result_number,
            ty: ty,
        })
    }

    pub fn expect_constant_integer(&self) -> i64 {
        if let Value::ConstantInteger { value, .. } = *self {
            value
        } else {
            panic!("expected a constant integer");
        }
    }

    pub fn expect_register_ref(&self) -> &RegisterRef {
        if let Value::RegisterRef(ref r) = *self {
            r
        } else {
            panic!("expected a register reference");
        }
    }
}

impl std::fmt::Debug for Value
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Value::ConstantInteger { bit_width, value } => {
                write!(fmt, "i{} {}", bit_width, value)
            },
            Value::RegisterRef(ref reg) => write!(fmt, "%<reg:{}>", reg.register_id),
            Value::ArgumentRef { id, .. } => write!(fmt, "%<arg:{}>", id),
        }
    }
}

