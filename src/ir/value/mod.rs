
pub use self::value::{Value,ValueTrait};

pub use self::pointer::Pointer;
pub use self::register::Register;
pub use self::literal::{Literal,LiteralTrait};
pub use self::globalref::GlobalRef;
pub use self::blockref::BlockRef;
pub use self::functionref::FunctionRef;

pub mod pointer;
pub mod register;
pub mod literal;
pub mod globalref;
pub mod blockref;
pub mod functionref;

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
    }

    #[derive(Clone,Debug,PartialEq,Eq)]
    pub enum Value
    {
        Literal(value::Literal),
        Pointer(value::Pointer),
        Register(value::Register),
        Instruction(ir::Instruction),

        GlobalRef(value::GlobalRef),
        BlockRef(value::BlockRef),
        FunctionRef(value::FunctionRef),
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

        pub fn global_ref(global: &ir::Global) -> Value {
            value::GlobalRef::reference(global).into()
        }

        pub fn block_ref(block: &ir::Block) -> Value {
            value::BlockRef::reference(block).into()
        }

        pub fn function_ref(func: &ir::Function) -> Value {
            value::FunctionRef::reference(func).into()
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

        /// Checks if the value is an instruction.
        pub fn is_instruction(&self) -> bool {
            if let &Value::Instruction(..) = self {
                true
            } else {
                false
            }
        }
    }

    impl lang::Value for Value
    {
        type Type = Type;

        fn subvalues(&self) -> Vec<Self> {
            match self {
                &ir::Value::Instruction(ref i) => i.subvalues(),
                _ => Vec::new(),
            }
        }

        fn map_subvalues<F>(self, f: F) -> Self
            where F: FnMut(Self) -> Self {
            match self {
                Value::Instruction(i) => i.map_subvalues(f),
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

        fn is_terminator(&self) -> bool {
            // only instructions can be terminators
            if let &ir::Value::Instruction(ref inst) = self {
                inst.is_terminator()
            } else {
                false
            }
        }

        fn ty(&self) -> Type {
             match self {
                &Value::Literal(ref val) => val.ty(),
                &Value::Pointer(ref val) => val.ty(),
                &Value::Register(ref val) => val.ty(),
                &Value::Instruction(ref val) => val.ty(),
                &Value::GlobalRef(ref val) => val.ty(),
                &Value::BlockRef(ref val) => val.ty(),
                &Value::FunctionRef(ref val) => val.ty(),
            }
        }
    }

    impl ValueTrait for Value { }

    impl fmt::Display for Value
    {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
            match self {
                &Value::Literal(ref val) => val.fmt(fmt),
                &Value::Pointer(ref val) => val.fmt(fmt),
                &Value::Register(ref val) => val.fmt(fmt),
                &Value::Instruction(ref val) => val.fmt(fmt),
                &Value::GlobalRef(ref val) => val.fmt(fmt),
                &Value::BlockRef(ref val) => val.fmt(fmt),
                &Value::FunctionRef(ref val) => val.fmt(fmt),
            }
        }
    }
}
