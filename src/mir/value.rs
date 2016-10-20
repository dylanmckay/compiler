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

/// A constant integer.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct ConstantInteger {
    pub bit_width: u32,
    pub value: i64,
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
    ConstantInteger(ConstantInteger),
    /// A register.
    RegisterRef(RegisterRef),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ValueInfo
{
    Input,
    Output,
    InputOutput,
}

impl Value
{
    /// Gets the type of the value.
    pub fn ty(&self) -> Type {
        match *self {
            Value::ArgumentRef { ref ty, .. } => ty.clone(),
            Value::ConstantInteger(ref c) => Type::Integer { bit_width: c.bit_width },
            Value::RegisterRef(ref reg)  => reg.ty.clone(),
        }
    }

    /// Creates a new n-bit constant integer.
    pub fn i(width: u32, value: i64) -> Self {
        Value::ConstantInteger(ConstantInteger {
            bit_width: width,
            value: value,
        })
    }

    /// Creates a new register reference.
    pub fn register_ref(register_id: util::Id, result_number: u32, ty: Type) -> Self {
        Value::RegisterRef(RegisterRef {
            register_id: register_id,
            result_number: result_number,
            ty: ty,
        })
    }

    pub fn is_register_ref(&self) -> bool {
        if let Value::RegisterRef(..) = *self { true } else { false}
    }

    pub fn expect_constant_integer(&self) -> &ConstantInteger {
        if let Value::ConstantInteger(ref c) = *self {
            c
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
            Value::ConstantInteger(ref c) => {
                write!(fmt, "i{} {}", c.bit_width, c.value)
            },
            Value::RegisterRef(ref reg) => write!(fmt, "%<reg:{}>", reg.register_id),
            Value::ArgumentRef { id, .. } => write!(fmt, "%<arg:{}>", id),
        }
    }
}

