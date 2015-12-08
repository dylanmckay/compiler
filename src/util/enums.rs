#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub enum ByteOrder
{
    LittleEndian,
    BigEndian,
}

/// Specified the signedness of an integer.
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub enum IntegerKind
{
    Signed,
    Unsigned,
}

impl IntegerKind
{
    pub fn prefix(self) -> char {
        match self {
            IntegerKind::Signed => 'i',
            IntegerKind::Unsigned => 'u',
        }
    }
}

/// Specified the signedness of a number.
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub enum Sign
{
    Plus,
    Minus,
}

impl Sign
{
    pub fn sign(self) -> char {
        match self {
            Sign::Plus => '+',
            Sign::Minus => '-',
        }
    }

    pub fn sign_if_minus(self) -> Option<char> {
        if let Sign::Minus = self {
            Some('-')
        } else {
            None
        }
    }
}
