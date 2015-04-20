
/// A possible immuatable reference to something.
pub enum Reference<'a, T> = Option<&'a T>;

pub mod types
{
    pub enum Primitive
    {
        Integer(Integer),
        Float(Float),
    }

    pub enum IntegerKind
    {
        Signed,
        Unsigned,
    }

    /// An integer(kind, size).
    pub enum Integer(IntegerKind, u16);

    pub enum Float(u16);

    /// A type.
    pub enum Type<'a>
    {
        Primitive(Primitive),
        Class(Class),
    }

}

pub mod values
{
    pub trait ValueTrait<'a>
    {
        fn ty(&self) -> Reference<'a, types::Type>;
    }

    pub enum Value
    {
        IntegerLiteral(IntegerLiteral),
        FloatLiteral(FloatLiteral),
    }

    pub struct IntegerLiteral(i64, types::Integer);
    pub struct FloatLiteral(f64, types::Float);

    impl<'a> ValueTrait for IntegerLiteral<'a> {
        fn ty(&self) -> &'a Type {
            let &(_, ty) = self;
            ty
        }
    }

}

/// An identifier.
pub struct Identifier(String);

/// A class.
pub struct Class
{

}

pub struct Parser
{
   
}
