use Type;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct String
{
    text: ::std::string::String,
}

impl String
{
    pub fn new(text: ::std::string::String) -> Self {
        String {
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

impl_expression!(String);
