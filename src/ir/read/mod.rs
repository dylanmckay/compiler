pub use self::token::{Token,Tokenizer};
pub use self::parse::Parser;

pub mod token;
pub mod parse;

pub fn textual<I>(characters: I) -> Result<::ir::Module,String>
    where I: Iterator<Item=char> {
    Parser::new(characters).parse()
}
