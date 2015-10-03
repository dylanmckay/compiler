
pub use self::ty::{Type,TypeTrait};

pub use self::pointer::Pointer;
pub use self::array::Array;
pub use self::decimal::Decimal;
pub use self::integer::Integer;
pub use self::label::Label;
pub use self::function::Function;
pub use self::strukt::Struct;
pub use self::vector::Vector;
pub use self::void::Void;


#[macro_use]
pub mod ty
{
    use ir::types::{Void,Pointer,Integer,Decimal,Vector,Array,Struct,Function,Label};
    use lang;
    use util::IntegerKind;

    use std::fmt;

    pub trait TypeTrait : Clone + Eq + fmt::Display + lang::Type + Into<Type>
    {
        /// Gets the size of the type in bits.
        /// 
        /// If the size is zero, the object can only exist through a pointer.
        fn size(&self) -> u64 { 0 }

        /// Checks if a type can exist on its own.
        ///
        /// Physical types must be representable in memory.
        /// For example, you cannot have an instance of a Function,
        /// but you can have an instance of a pointer to a function.
        ///
        /// Non-physical types have sizes of zero.
        fn is_physical(&self) -> bool {
            self.size() != 0
        }
    }

    #[derive(Clone,Eq,PartialEq,Debug)]
    pub enum Type
    {
        Void(Void),

        Pointer(Pointer),
        Integer(Integer),
        Decimal(Decimal),

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

        /// A pointer to a type.
        pub fn pointer(ty: Type) -> Type {
            Pointer::to(ty).into()
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

        /// Creates a new decimaling point type.
        pub fn decimal(bit_width: u16) -> Type {
            Type::Decimal(Decimal::new(bit_width))
        }

        /// Creates a new decimaling point type.
        /// Alias of `Type::decimal`.
        pub fn f(bit_width: u16) -> Type {
            Type::decimal(bit_width)
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
                &Type::Pointer(ref ty) => ty.size(),
                &Type::Integer(ref ty) => { ty.size() },
                &Type::Decimal(ref ty) => { ty.size() },
                &Type::Vector(ref ty) => { ty.size() },
                &Type::Array(ref ty) => { ty.size() },
                &Type::Struct(ref ty) => { ty.size() },
                &Type::Function(ref ty) => { ty.size() },
                &Type::Label(ref ty) => { ty.size() },
            }
        }
    }

    impl fmt::Display for Type
    {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error>
        {
            match self {
                &Type::Void(ty) => ty.fmt(fmt),
                &Type::Pointer(ref ty) => ty.fmt(fmt),
                &Type::Integer(ref ty) => ty.fmt(fmt),
                &Type::Decimal(ref ty) => ty.fmt(fmt),
                &Type::Vector(ref ty) => ty.fmt(fmt),
                &Type::Array(ref ty) => ty.fmt(fmt),
                &Type::Struct(ref ty) => ty.fmt(fmt),
                &Type::Function(ref ty) => ty.fmt(fmt),
                &Type::Label(ref ty) => ty.fmt(fmt),
            }
        }
    }

    impl lang::Type for Type { }

    macro_rules! impl_type {
        ($ty:ident) => {

            impl_into_type!($ty);

            impl ::lang::Type for $ty { }
        }
    }

    macro_rules! impl_into_type {
        ($ty:ident) => {
            impl Into<::ir::Type> for $ty
            {
                fn into(self) -> ::ir::Type {
                    ::ir::Type::$ty(self)
                }
            }

        }
    }
}

pub mod pointer;
pub mod array;
pub mod decimal;
pub mod integer;
pub mod label;
pub mod function;
pub mod strukt;
pub mod vector;
pub mod void;

