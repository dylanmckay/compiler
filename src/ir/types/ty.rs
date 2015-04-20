
use ir::types::{Void, Integer, Float, Vector, Array, Struct, Function, Label};
use lang;
use util::IntegerKind;

use std::fmt;

pub trait TypeTrait : Clone + Eq + PartialEq + fmt::Display + lang::Type
{
    /// Gets the size of the type in bits.
    fn size(&self) -> u64;

    fn upcast(self) -> Type;
}

#[derive(Clone,Eq,PartialEq,Debug)]
pub enum Type
{
    Void(Void),

    Integer(Integer),
    Float(Float),

    Vector(Vector),
    Array(Array),
    Struct(Struct),

    Function(Function),
    Label(Label),
}

impl Type
{
    pub fn void() -> Type { Type::Void(Void::void()) }

    pub fn integer(kind: IntegerKind, bit_width: u16) -> Type {
        Type::Integer(Integer::new(kind, bit_width))
    }

    /// Creates a signed integer/
    pub fn i(bit_width: u16) -> Type { Type::integer(IntegerKind::Signed, bit_width) }
    /// Creates an unsigned integer.
    pub fn u(bit_width: u16) -> Type { Type::integer(IntegerKind::Unsigned, bit_width) }

    pub fn i8() ->   Type { Type::i(8)   }
    pub fn i16() ->  Type { Type::i(16)  }
    pub fn i32() ->  Type { Type::i(32)  }
    pub fn i64() ->  Type { Type::i(64)  }
    pub fn i128() -> Type { Type::i(128) }

    pub fn u8() ->   Type { Type::u(8)   }
    pub fn u16() ->  Type { Type::u(16)  }
    pub fn u32() ->  Type { Type::u(32)  }
    pub fn u64() ->  Type { Type::u(64)  }
    pub fn u128() -> Type { Type::u(128) }

    /// Creates a new floating point type.
    pub fn float(bit_width: u16) -> Type {
        Type::Float(Float::new(bit_width))
    }

    /// Creates a new floating point type.
    /// Alias of `Type::float`.
    pub fn f(bit_width: u16) -> Type {
        Type::float(bit_width)
    }

    pub fn f16() -> Type { Type::f(16) }
    pub fn f32() -> Type { Type::f(32) }
    pub fn f64() -> Type { Type::f(64) }

    pub fn vector(count: u64, ty: Type) -> Type {
        Type::Vector(Vector::new(count,ty))
    }

    pub fn array(count: u64, ty: Type) -> Type {
        Type::Array(Array::new(count,ty))
    }

    /// Creates a new unit struct.
    pub fn unit_struct() -> Type { Type::Struct(Struct::unit()) }

    /// Creates a new label type.
    pub fn label() -> Type { Type::Label(Label::new()) }
}

impl TypeTrait for Type
{
    fn size(&self) -> u64 {
        match self {
            &Type::Void(ref ty) => { ty.size() },
            &Type::Integer(ref ty) => { ty.size() },
            &Type::Float(ref ty) => { ty.size() },
            &Type::Vector(ref ty) => { ty.size() },
            &Type::Array(ref ty) => { ty.size() },
            &Type::Struct(ref ty) => { ty.size() },
            &Type::Function(ref ty) => { ty.size() },
            &Type::Label(ref ty) => { ty.size() },
        }
    }

    fn upcast(self) -> Type {
        self
    }
}

impl fmt::Display for Type
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error>
    {
        match self {
            &Type::Void(ty) => {
                ty.fmt(fmt)
            },
            &Type::Integer(ref ty) => {
                ty.fmt(fmt)
            },
            &Type::Float(ref ty) => {
                ty.fmt(fmt)
            },
            &Type::Vector(ref ty) => {
                ty.fmt(fmt)
            },
            &Type::Array(ref ty) => {
                ty.fmt(fmt)
            },
            &Type::Struct(ref ty) => {
                ty.fmt(fmt)
            },
            &Type::Function(ref ty) => {
                ty.fmt(fmt)
            },
            &Type::Label(ref ty) => {
                ty.fmt(fmt)
            },
        }
    }
}

impl lang::Type for Type
{
}
