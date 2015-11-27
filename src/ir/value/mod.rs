
pub use self::value::{Expression,ExpressionTrait};

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

    pub trait ExpressionTrait : Clone + fmt::Debug + Into<Expression>
    {
    }

    #[derive(Clone,Debug,PartialEq,Eq)]
    pub enum Expression
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

    impl Expression
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
        pub fn integer<T: ToBigInt>(ty: types::Integer, val: T) -> Option<Expression> {
            value::Literal::integer(ty,val).map(|i| i.into())
        }

        pub fn decimal(ty: types::Decimal, bits: BitVec) -> Expression {
            value::Literal::decimal(ty,bits).into()
        }

        pub fn strukt(fields: Vec<Expression>) -> Expression {
            value::Literal::strukt(fields).into()
        }

        pub fn unit_struct() -> Expression {
            value::Literal::unit_struct().into()
        }

        pub fn global_ref(global: &ir::Global) -> Expression {
            value::GlobalRef::reference(global).into()
        }

        pub fn block_ref(block: &ir::Block) -> Expression {
            value::BlockRef::reference(block).into()
        }

        pub fn function_ref(func: &ir::Function) -> Expression {
            value::FunctionRef::reference(func).into()
        }

        pub fn register_ref(reg: &ir::value::Register) -> Expression {
            value::RegisterRef::reference(reg).into()
        }

        /// Creates an unnamed register.
        pub fn register<V>(value: V) -> Expression
            where V: Into<Expression> {
            value::Register::unnamed(value.into()).into()
        }

        /// Creates a named register.
        pub fn register_named<I>(name: I, value: Expression) -> Expression
            where I: Into<String> {

            let name = ir::Name::named(name);
            value::Register::new(name, value).into()
        }

        pub fn as_literal(&self) -> Option<&value::Literal> {
            match *self {
                Expression::Literal(ref v) => Some(v),
                _ => None,
            }
        }

        pub fn is_literal(&self) -> bool {
            match *self {
                Expression::Literal(..) => true,
                _ => false,
            }
        }

        /// Checks if the value is an instruction.
        pub fn is_instruction(&self) -> bool {
            match *self {
                Expression::Instruction(..) => true,
                _ => false,
            }
        }

        pub fn is_function_ref(&self) -> bool {
            match *self {
                Expression::FunctionRef(..) => true,
                _ => false,
            }
        }

        pub fn is_block_ref(&self) -> bool {
            match *self {
                Expression::BlockRef(..) => true,
                _ => false,
            }
        }

        pub fn ty(&self) -> ir::Type {
            lang::Value::ty(self)
        }
    }

    impl lang::Value for Expression
    {
        type Type = Type;

        // FIXME: subvalue support is patchy

        fn subvalues(&self) -> Vec<&Self> {
            match *self {
                ir::Expression::Instruction(ref i) => i.subvalues(),
                _ => Vec::new(),
            }
        }

        fn map_subvalues<F>(self, f: F) -> Self
            where F: FnMut(Self) -> Self {
            match self {
                Expression::Instruction(i) => i.map_subvalues(f).into(),
                _ => self,
            }
        }

        fn flatten(self, block: &mut ir::Block) -> Self {
            // only instructions need flattening
            if let Expression::Instruction(i) = self {
                i.flatten(block).into()
            } else {
                self
            }
        }

        fn is_single_critical(&self) -> bool {
            match *self {
                ir::Expression::Literal(..) => false,
                ir::Expression::Instruction(ref i) => i.is_single_critical(),
                _ => true,
            }
        }

        fn is_terminator(&self) -> bool {
            // only instructions can be terminators
            if let ir::Expression::Instruction(ref inst) = *self {
                inst.is_terminator()
            } else {
                false
            }
        }

        fn is_simple(&self) -> bool {
             match *self {
                 Expression::Literal(..) => true,
                 Expression::Pointer(ref val) => val.underlying().is_simple(),
                 Expression::Register(..) => true,
                 Expression::Instruction(..) => false,
                 Expression::GlobalRef(..) => true,
                 Expression::BlockRef(..) => true,
                 Expression::FunctionRef(..) => true,
                 Expression::RegisterRef(..) => true,
             }
        }

        fn ty(&self) -> Type {
             match *self {
                Expression::Literal(ref val) => val.ty(),
                Expression::Pointer(ref val) => val.ty(),
                Expression::Register(ref val) => val.ty(),
                Expression::Instruction(ref val) => val.ty(),
                Expression::GlobalRef(ref val) => val.ty(),
                Expression::BlockRef(ref val) => val.ty(),
                Expression::FunctionRef(ref val) => val.ty(),
                Expression::RegisterRef(ref val) => val.ty(),
            }
        }
    }

    impl ExpressionTrait for Expression { }

    /// Implements Into<Expression> for u8,i32,etc
    macro_rules! impl_into_value_for_integers {
        (
            $( $ty:ident ),*
        ) => {
            $(
                impl Into<Expression> for $ty
                {
                    fn into(self) -> Expression {
                        Expression::$ty(self)
                    }
                }
            )*
        }
    }

    impl_into_value_for_integers!(u8,u16,u32,u64,i8,i16,i32,i64);
}
