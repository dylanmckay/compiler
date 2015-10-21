
pub use self::value::{Value,ValueTrait};

pub use self::pointer::Pointer;
pub use self::register::Register;
pub use self::literal::{Literal,LiteralTrait};
pub use self::globalref::GlobalRef;
pub use self::blockref::BlockRef;
pub use self::functionref::FunctionRef;
pub use self::registerref::RegisterRef;

pub mod pointer;
pub mod register;
pub mod literal;
pub mod globalref;
pub mod blockref;
pub mod functionref;
pub mod registerref;

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
        RegisterRef(value::RegisterRef),
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

        pub fn register_ref(reg: &ir::value::Register) -> Value {
            value::RegisterRef::reference(reg).into()
        }

        /// Creates an unnamed register.
        pub fn register<V>(value: V) -> Value
            where V: Into<Value> {
            value::Register::unnamed(value.into()).into()
        }

        /// Creates a named register.
        pub fn register_named<I>(name: I, value: Value) -> Value
            where I: Into<String> {

            let name = ir::Name::named(name);
            value::Register::new(name, value).into()
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

        pub fn ty(&self) -> ir::Type {
            lang::Value::ty(self)
        }
    }

    impl lang::Value for Value
    {
        type Type = Type;

        // FIXME: subvalue support is patchy

        fn subvalues(&self) -> Vec<&Self> {
            match self {
                &ir::Value::Instruction(ref i) => i.subvalues(),
                _ => Vec::new(),
            }
        }

        fn map_subvalues<F>(self, f: F) -> Self
            where F: FnMut(Self) -> Self {
            match self {
                Value::Instruction(i) => i.map_subvalues(f).into(),
                _ => self,
            }
        }

        fn flatten(self, block: &mut ir::Block) -> Self {
            // only instructions need flattening
            if let Value::Instruction(i) = self {
                i.flatten(block).into()
            } else {
                self
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
                &Value::RegisterRef(ref val) => val.ty(),
            }
        }
    }

    impl ValueTrait for Value { }

    /// Implements Into<Value> for u8,i32,etc
    macro_rules! impl_into_value_for_integers {
        (
            $( $ty:ident ),*
        ) => {
            $(
                impl Into<Value> for $ty
                {
                    fn into(self) -> Value {
                        Value::$ty(self)
                    }
                }
            )*
        }
    }

    impl_into_value_for_integers!(u8,u16,u32,u64,i8,i16,i32,i64);
}
