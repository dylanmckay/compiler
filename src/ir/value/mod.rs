
pub use self::value::{Value,ValueTrait};

pub use self::global::Global;
pub use self::pointer::Pointer;
pub use self::constant::{Constant,ConstantTrait};

pub mod global;
pub mod pointer;
pub mod constant;

pub mod value
{
    use ir::{self,types,value,Type};
    use bit_vec::BitVec;
    use std::fmt;
    use lang;
    use util;

    use num::bigint::ToBigInt;

    pub trait ValueTrait : Clone + fmt::Display + fmt::Debug + Into<Value>
    {
        fn ty(&self) -> Type;
    }

    #[derive(Clone,Debug)]
    pub enum Value
    {
        Constant(value::Constant),
        Global(value::Global),
        Pointer(value::Pointer),

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
            value::Constant::integer(ty,val).map(|i| i.into())
        }

        pub fn decimal(ty: types::Decimal, bits: BitVec) -> Value {
            value::Constant::decimal(ty,bits).into()
        }

        pub fn strukt(fields: Vec<Value>) -> Value {
            value::Constant::strukt(fields).into()
        }

        pub fn unit_struct() -> Value {
            value::Constant::unit_struct().into()
        }

        pub fn as_constant(&self) -> Option<&value::Constant> {
            match self {
                &Value::Constant(ref v) => Some(v),
                _ => None,
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
                &ir::Value::Constant(..) => false,
                &ir::Value::Instruction(ref i) => i.is_single_critical(),
                _ => true,
            }
        }
    }

    impl ValueTrait for Value
    {
        fn ty(&self) -> Type {
            match self {
                &Value::Constant(ref val) => val.ty(),
                &Value::Global(ref val) => val.ty(),
                &Value::Pointer(ref val) => val.ty(),
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
                &Value::Constant(ref val) => val.fmt(fmt),
                &Value::Global(ref val) => val.fmt(fmt),
                &Value::Pointer(ref val) => val.fmt(fmt),
                &Value::Instruction(ref val) => val.fmt(fmt),
                &Value::Block(ref val) => val.fmt(fmt),
                &Value::Function(ref val) => val.fmt(fmt),
            }
        }
    }
}
