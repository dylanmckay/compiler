pub use self::characters::Characters;
pub use self::token::Token;
pub use self::tokenizer::Tokenizer;
pub use self::parse::Parser;

pub mod characters;
pub mod token;
pub mod tokenizer;
pub mod parse;

pub fn textual<I>(characters: I) -> Result<::ir::Module,String>
    where I: Iterator<Item=char> {
    Parser::new(characters).parse()
}
