
pub use self::value::{Value,ValueTrait};

pub use self::global::Global;
pub use self::pointer::Pointer;
pub use self::register::Register;
pub use self::literal::{Literal,LiteralTrait};

pub mod global;
pub mod pointer;
pub mod register;
pub mod literal;

pub mod value
{
    use ir::{self,types,value,Type};
    use bit_vec::BitVec;
    use std::fmt;
    use lang;
    use util;

    use num::bigint::ToBigInt;

    pub trait ValueTrait : Clone + fmt::Debug + Into<Value>
    {
        fn ty(&self) -> Type;
    }

    #[derive(Clone,Debug)]
    pub enum Value
    {
        Literal(value::Literal),
        Global(value::Global),
        Pointer(value::Pointer),
        Register(value::Register),

        Instruction(ir::Instruction),
        Block(ir::Block),
        Function(ir::Function),
    }

    impl Value
    {
        /// Creates a signed integer value.
        pub fn i<T: ToBigInt>(bit_width: u16, value: T) -> Self {
            let ty = types::Integer::new(util::IntegerKind::Signed, bit_width);
            Self::integer(ty, value).unwrap()
        }

        /// Creates an unsigned integer value.
        pub fn u<T: ToBigInt>(bit_width: u16, value: T) -> Self {
           let ty = types::Integer::new(util::IntegerKind::Unsigned, bit_width);
           Self::integer(ty, value).unwrap()
        }

        pub fn u8(value: u8)   -> Self { Self::u(8, value) }
        pub fn u16(value: u16) -> Self { Self::u(16, value) }
        pub fn u32(value: u32) -> Self { Self::u(32, value) }
        pub fn u64(value: u64) -> Self { Self::u(64, value) }
        pub fn i8(value: i8)   -> Self { Self::i(8, value) }
        pub fn i16(value: i16) -> Self { Self::i(16, value) }
        pub fn i32(value: i32) -> Self { Self::i(32, value) }
        pub fn i64(value: i64) -> Self { Self::i(64, value) }

        /// Creates an integer, returning `None` if `val` cannot fit into `ty`.
        pub fn integer<T: ToBigInt>(ty: types::Integer, val: T) -> Option<Value> {
            value::Literal::integer(ty,val).map(|i| i.into())
        }

        pub fn decimal(ty: types::Decimal, bits: BitVec) -> Value {
            value::Literal::decimal(ty,bits).into()
        }

        pub fn strukt(fields: Vec<Value>) -> Value {
            value::Literal::strukt(fields).into()
        }

        pub fn unit_struct() -> Value {
            value::Literal::unit_struct().into()
        }

        /// Creates an unnamed register.
        pub fn register(ty: ir::Type) -> Value {
            value::Register::unnamed(ty).into()
        }

        /// Creates a named register.
        pub fn register_named<I>(name: I, ty: ir::Type) -> Value
            where I: Into<String> {

            let name = ir::Name::named(name);
            value::Register::new(name, ty).into()
        }

        pub fn as_literal(&self) -> Option<&value::Literal> {
            match self {
                &Value::Literal(ref v) => Some(v),
                _ => None,
            }
        }

        pub fn is_literal(&self) -> bool {
            if let &Value::Literal(..) = self {
                true
            } else {
                false
            }
        }
    }

    impl lang::Value for Value
    {
        fn subvalues(&self) -> Vec<Self> {
            use lang::{Value,Block};

            match self {
                &ir::Value::Instruction(ref i) => i.subvalues(),
                &ir::Value::Block(ref i) => i.subvalues(),
                _ => Vec::new(),
            }
        }

        fn map_subvalues<F>(self, f: F) -> Self
            where F: FnMut(Self) -> Self {
            use lang::Block;

            match self {
                Value::Instruction(i) => i.map_subvalues(f),
                Value::Block(i) => i.map_subvalues(f).into(),
                _ => self,
            }
        }

        fn is_single_critical(&self) -> bool {
            match self {
                &ir::Value::Literal(..) => false,
                &ir::Value::Instruction(ref i) => i.is_single_critical(),
                _ => true,
            }
        }
    }

    impl ValueTrait for Value
    {
        fn ty(&self) -> Type {
            match self {
                &Value::Literal(ref val) => val.ty(),
                &Value::Global(ref val) => val.ty(),
                &Value::Pointer(ref val) => val.ty(),
                &Value::Register(ref val) => val.ty(),
                &Value::Instruction(ref val) => val.ty(),
                &Value::Block(ref val) => val.ty(),
                &Value::Function(ref val) => val.ty(),
            }
        }
    }

    impl fmt::Display for Value
    {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
            match self {
                &Value::Literal(ref val) => val.fmt(fmt),
                &Value::Global(ref val) => val.fmt(fmt),
                &Value::Pointer(ref val) => val.fmt(fmt),
                &Value::Register(ref val) => val.fmt(fmt),
                &Value::Instruction(ref val) => val.fmt(fmt),
                &Value::Block(ref val) => val.name().fmt(fmt),
                &Value::Function(ref val) => val.fmt(fmt),
            }
        }
    }
}
