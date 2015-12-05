use super::{Token,Characters};
use util;

use std;
use std::error::Error;

pub type Result<T> = std::result::Result<T,String>;

/// A list of symbols to be tokenized.
///
/// They are sorted from most specific to least.
/// This means that `+=` will precede `+` etc.
pub const SYMBOL_LIST: &'static [&'static str] = &[
    // Symbols with two characters
    "->",
    // Symbols with a single character
    ",", ":", "(", ")", "@", "%", "{", "}", "=",
];

pub struct Tokenizer<I: Iterator<Item=char>>
{
    chars: Characters<I>,

    preserve_comments: bool,

    peek_buf: Option<Token>,
}

impl<I> Tokenizer<I>
    where I: Iterator<Item=char>
{
    pub fn new(chars: I) -> Self {
        Tokenizer {
            chars: Characters::new(chars),
            preserve_comments: false,
            peek_buf: None,
        }
    }

    /// Allows the tokenizer to preserve comments it tokenizes.
    pub fn preserve_comments(mut self) -> Self {
        self.preserve_comments = true;
        self
    }

    pub fn peek(&mut self) -> Option<Result<Token>> {
        let token = match self.next() {
            Some(Ok(token)) => token,
            Some(Err(token)) => return Some(Err(token)),
            None => return None,
        };

        assert!(self.peek_buf.is_none());

        self.peek_buf = Some(token.clone());
        Some(Ok(token))
    }

    pub fn eat(&mut self) {
        self.next();
    }

    pub fn eat_while<P>(&mut self, mut predicate: P) -> Result<()>
        where P: FnMut(Token) -> bool {
        loop {
            match self.peek() {
                Some(Ok(tok)) => {
                    if predicate(tok) {
                        self.eat()
                    } else {
                        break;
                    }
                },
                Some(Err(e)) => return Err(e),
                None => break,
            }
        }

        Ok(())
    }

    fn eat_whitespace(&mut self) {
        self.chars.eat_while(|c| c.is_whitespace() && c != '\n')
    }

    fn next_string(&mut self) -> Option<Result<Token>> {
        self.assert('"');
        let string = self.chars.consume_while(|c| c != '\"');
        self.assert('"');

        Some(Ok(Token::String(string.collect())))
    }

    fn next_integer(&mut self) -> Option<Result<Token>> {
        let string: String = self.chars.consume_while(|c| c.is_numeric()).collect();

        let int = match i64::from_str_radix(&string, 10) {
            Ok(int) => int,
            Err(e) => return Some(Err(format!("Could not parse integer: {}",
                                              e.description()))),
        };

        Some(Ok(Token::Integer(int)))
    }

    fn next_word(&mut self) -> Option<Result<Token>> {
        let word: String = self.chars.consume_while(internal::can_word_contain).collect();

        Some(Ok(Token::Word(word)))
    }

    fn next_symbol(&mut self) -> Option<Result<Token>> {
        use std::borrow::Borrow;

        let first_char = match self.expect_something() {
            Ok(c) => c,
            Err(e) => return Some(Err(e)),
        };

        let second_char = self.chars.peek();

        let mut sym_str = format!("{}", first_char);

        if let Some(c) = second_char {
            sym_str.push(c)
        }

        while !sym_str.is_empty() {
            if SYMBOL_LIST.contains(&sym_str.borrow()) {

                // We only peek the second character, not consume it
                // if we matched with two characters, then consume it.
                if sym_str.len() == 2 {
                    self.chars.eat();
                }

                return Some(Ok(Token::symbol(sym_str)));
            } else {
                sym_str.pop();
            }
        }

        Some(Err(format!("unknown token: {}", first_char)))
    }

    fn next_comment(&mut self) -> Option<Result<Token>> {
        self.assert(';');
        let text: String = self.chars.consume_while(|c| c != '\n').collect();

        Some(Ok(Token::comment(text)))
    }

    fn eat_next_comment(&mut self) -> Option<Result<()>> {
        self.next_comment().map(|result| result.map(|_| ()))
    }

    fn assert(&mut self, expected: char) -> char {
        self.expect(expected).expect("unexpected character")
    }

    fn expect(&mut self, expected: char) -> Result<char> {
        // TODO: more specific error message
        self.expect_one_of(&[expected])
    }

    fn expect_something(&mut self) -> Result<char> {
        match self.chars.next() {
            Some(c) => Ok(c),
            None => Err(format!("expected a token but found nothing")),
        }
    }

    fn expect_one_of(&mut self, expected: &[char]) -> Result<char> {
        let expected_str = util::comma_separated_values(expected.iter());

        let next = match self.chars.next() {
            Some(c) => c,
            None => {
                return Err(format!("expected one of {} but got nothing", expected_str))
            },
        };

        if expected.iter().any(|&c| c == next) {
            Ok(next)
        } else {
            Err(format!("Expected one of {} but got {}", expected_str, next))
        }
    }
}

impl<I> Iterator for Tokenizer<I>
    where I: Iterator<Item=char>
{
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Result<Token>> {
        if self.peek_buf.is_some() {
            let token = self.peek_buf.clone().unwrap();
            self.peek_buf = None;
            return Some(Ok(token));
        }

        self.eat_whitespace();

        let first_char = match self.chars.peek() {
            Some(c) => c,
            None => return None,
        };

        if first_char == '"' {
            self.next_string()
        } else if first_char == '\n' {
            self.chars.eat();
            Some(Ok(Token::new_line()))
        } else if first_char == ';' {
            if self.preserve_comments {
                self.next_comment()
            } else {

                // TODO: This could be nicer
                match self.eat_next_comment() {
                    Some(Ok(..)) => (),
                    Some(Err(e)) => return Some(Err(e)),
                    None => panic!("expected a comment to be parsed"),
                }

                // TODO: fix needless recursion
                self.next()
            }
        } else if first_char.is_numeric() {
            self.next_integer()
        } else if internal::can_word_start_with(first_char) {
            self.next_word()
        } else {
            // Try to parse the token as a symbol.
            // This is our final fallback.
            self.next_symbol()
        }
    }
}

mod internal
{
    pub fn can_word_start_with(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    pub fn can_word_contain(c: char) -> bool {
        can_word_start_with(c) || c.is_numeric()
    }
}

#[allow(unused_imports)]
mod test
{
    use ir::read::{Token,Tokenizer};

    /// Expects a mapping from a string of tokens into
    /// a list of tokens with the default tokenizer.
    macro_rules! expect_mapping {
        ($input:expr => $( $output:expr ),* ) => {
            expect_mapping_with!(Tokenizer::new($input.chars()) =>
                                 $( $output ),*);
        }
    }

    /// Expects a mapping from a string of tokens into
    /// a list of tokens, given a specific tokenizer.
    macro_rules! expect_mapping_with {
        // TODO: make the macro take several arguments
        // for several outputs
        ($tokenizer:expr => $( $output:expr ),* ) => {
            {
                let mut tokenizer = $tokenizer;

                $(
                    let token = tokenizer.next().unwrap().unwrap();
                    assert!($output == token,
                            format!("expected {} but got {}",
                                    $output, token));
                )*
            }
        }
    }

    #[test]
    fn test_string() {
        expect_mapping!("\"hello\"" =>  Token::string("hello"),
                                        Token::new_line());

        expect_mapping!("\"hello abc\"" => Token::string("hello abc"),
                                           Token::new_line());

        expect_mapping!("\"hello world\"  \"it is me\"" =>
                        Token::string("hello world"),
                        Token::string("it is me"),
                        Token::new_line());
    }

    #[test]
    fn test_integer() {
        expect_mapping!("123" => Token::integer(123),
                                 Token::new_line());

        expect_mapping!("0982" => Token::integer(982),
                                  Token::new_line());

        expect_mapping!("333 662" => Token::integer(333),
                                     Token::integer(662),
                                     Token::new_line());

        expect_mapping!("1 2 3 4" => Token::integer(1),
                                     Token::integer(2),
                                     Token::integer(3),
                                     Token::integer(4),
                                     Token::new_line());
    }

    #[test]
    fn test_word() {
        expect_mapping!("hello" => Token::word("hello"));

        expect_mapping!("ab cd" => Token::word("ab"),
                                   Token::word("cd"),
                                   Token::new_line());

        expect_mapping!("a b c d" => Token::word("a"),
                                     Token::word("b"),
                                     Token::word("c"),
                                     Token::word("d"),
                                     Token::new_line());
    }

    #[test]
    fn test_comments_not_preserved_by_default() {
        expect_mapping!("hello ; there" => Token::word("hello"),
                                           Token::new_line());

        expect_mapping!(";why hello" => Token::new_line());
    }

    #[test]
    fn test_comments() {
        expect_mapping_with!(Tokenizer::new("hello world ; this is me".chars())
                             .preserve_comments()
                             => Token::word("hello"),
                                Token::word("world"),
                                Token::comment(" this is me"),
                                Token::new_line());

        expect_mapping_with!(Tokenizer::new(";this is a test".chars())
                             .preserve_comments()
                             => Token::comment("this is a test"),
                                Token::new_line());
    }

    #[test]
    fn test_symbols() {
        expect_mapping!("(" => Token::symbol("("),
                               Token::new_line());

        expect_mapping!(")" => Token::symbol(")"));
        expect_mapping!("{ }" => Token::symbol("{"), Token::symbol("}"));
        expect_mapping!(":" => Token::symbol(":"));
        expect_mapping!("," => Token::symbol(","));
        expect_mapping!("@%" => Token::symbol("@"), Token::symbol("%"));
        expect_mapping!(":=" => Token::symbol(":"), Token::symbol("="));
        expect_mapping!("->" => Token::symbol("->"));
        expect_mapping!("(->)" => Token::symbol("("), Token::symbol("->"),
                                  Token::symbol(")"));
    }

    #[test]
    fn test_multiple() {
        expect_mapping!("12 bark \"earth\"" => Token::integer(12),
                                               Token::word("bark"),
                                               Token::string("earth"),
                                               Token::new_line());

        expect_mapping!("a      23 gg \"a\"" => Token::word("a"),
                                                Token::integer(23),
                                                Token::word("gg"),
                                                Token::string("a"),
                                                Token::new_line());

        expect_mapping!("4:3{2\n}" => Token::integer(4),
                                      Token::symbol(":"),
                                      Token::integer(3),
                                      Token::symbol("{"),
                                      Token::integer(2),
                                      Token::new_line(),
                                      Token::symbol("}"),
                                      Token::new_line());
    }

    #[test]
    fn test_whitespace() {
        expect_mapping!("a\tb\t123\n\"qwer\"\n" => Token::word("a"),
                                                   Token::word("b"),
                                                   Token::integer(123),
                                                   Token::new_line(),
                                                   Token::string("qwer"),
                                                   Token::new_line(),
                                                   Token::new_line());
    }

    #[test]
    fn test_peek() {
        let mut tokenizer = Tokenizer::new("a b c".chars());

        assert_eq!(tokenizer.peek().unwrap().unwrap(),
                   Token::word("a"));

        assert_eq!(tokenizer.peek().unwrap().unwrap(),
                   Token::word("a"));

        assert_eq!(tokenizer.next().unwrap().unwrap(),
                   Token::word("a"));

        assert_eq!(tokenizer.peek().unwrap().unwrap(),
                   Token::word("b"));

        assert_eq!(tokenizer.next().unwrap().unwrap(),
                   Token::word("b"));

        assert_eq!(tokenizer.next().unwrap().unwrap(),
                   Token::word("c"));

        assert_eq!(tokenizer.next().unwrap().unwrap(),
                   Token::new_line());
    }
}
