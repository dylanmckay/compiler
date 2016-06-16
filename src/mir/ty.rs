#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Type
{
    Integer { bit_width: u32 },
    Nothing,
}

impl Type
{
    pub fn i(width: u32) -> Self { Type::Integer { bit_width: width } }

    pub fn i8() -> Self { Self::i(8) }
    pub fn i16() -> Self { Self::i(16) }

    pub fn bit_width(&self) -> u32 {
        match *self {
            Type::Integer { bit_width } => bit_width,
            Type::Nothing => 0,
        }
    }

    pub fn is_nothing(&self) -> bool { *self == Type::Nothing }
}

