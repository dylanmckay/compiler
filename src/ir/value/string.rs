use {Expression,Type};
use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct String
{
    id: util::Id,

    text: ::std::string::String,
}

impl String
{
    pub fn new(text: ::std::string::String) -> Self {
        String {
            id: util::Id::next(),
            text: text,
        }
    }

    pub fn text(&self) -> &str { &self.text }

    pub fn ty(&self) -> Type {
        // FIXME: Handle unicode
        Type::array(self.byte_count(), Type::u8())
    }

    pub fn byte_count(&self) -> u64 {
        // account for null terminator
        self.text.len() as u64 + 1
    }
}

impl util::Identifiable for String
{
    fn get_id(&self) -> util::Id { self.id }
}

impl Into<Expression> for String
{
    fn into(self) -> Expression {
        Expression::String(self)
    }
}
