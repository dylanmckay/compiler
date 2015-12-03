
use util;
use std;
use std::error::Error;

pub type Result<T> = std::result::Result<T,String>;

/// A token.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Token
{
    Word(String),

    String(String),
    // TODO: use BigNum
    Integer(i64),

    Symbol(String),

    NewLine,
    EOF,
}

impl Token
{
    pub fn word<S>(word: S) -> Self
        where S: Into<String> {
        Token::Word(word.into())
    }

    pub fn string<S>(string: S) -> Self
        where S: Into<String> {
        Token::String(string.into())
    }

    pub fn integer<I>(integer: I) -> Self
        where I: Into<i64> {
        Token::Integer(integer.into())
    }

    pub fn symbol<S>(symbol: S) -> Self
        where S: Into<String> {
        Token::Symbol(symbol.into())
    }

    pub fn new_line() -> Self {
        Token::NewLine
    }

    pub fn eof() -> Self {
        Token::EOF
    }
}

/// An iterator over a set of characters.
///
/// This type abstracts over another iterator of characters,
/// making sure that the yielded characters will have a specific
/// format.
///
/// * There will always be a new line (`\n`) before the end of file.
pub struct Characters<I: Iterator<Item=char>>
{
    it: std::iter::Peekable<I>,
    finished: bool,
    peek_buf: Option<char>,
}

impl<I> Characters<I>
    where I: Iterator<Item=char>
{
    pub fn new(it: I) -> Self {
        Characters {
            it: it.peekable(),
            finished: false,
            peek_buf: None,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        if self.peek_buf.is_none() {
            self.peek_buf = self.next();
        }

        self.peek_buf
    }

    pub fn eat_while<P>(&mut self, predicate: P)
        where P: FnMut(char) -> bool {
        self.consume_while(predicate);
    }

    pub fn consume_while<P>(&mut self, mut predicate: P)
        -> std::vec::IntoIter<char>
        where P: FnMut(char) -> bool {
        let mut result = Vec::new();

        loop {
            let c = match self.peek() {
                Some(c) => c,
                None => break,
            };

            if predicate(c) {
                result.push(self.next().unwrap());
            } else {
                break;
            }
        }

        result.into_iter()
    }
}

impl<I> Iterator for Characters<I>
    where I: Iterator<Item=char>
{
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.peek_buf.is_some() {
            let c = self.peek_buf.unwrap();
            self.peek_buf = None;
            return Some(c);
        }

        match self.it.next() {
            Some(c) => Some(c),
            None => {
                if self.finished {
                    None
                } else { // When we first reach the EOF
                    self.finished = true;
                    Some('\n')
                }
            },
        }
    }
}

pub struct Tokenizer<I: Iterator<Item=char>>
{
    chars: Characters<I>,
}

impl<I> Tokenizer<I>
    where I: Iterator<Item=char>
{
    pub fn new(chars: I) -> Self {
        Tokenizer {
            chars: Characters::new(chars),
        }
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
        unimplemented!();
    }

    fn assert(&mut self, expected: char) -> char {
        self.expect(expected).expect("unexpected character")
    }

    fn expect(&mut self, expected: char) -> Result<char> {
        // TODO: more specific error message
        self.expect_one_of(&[expected])
    }

    fn assert_one_of(&mut self, expected: &[char]) -> char {
        self.expect_one_of(expected)
            .expect("unexpected character")
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
        self.eat_whitespace();

        let first_char = match self.chars.peek() {
            Some(c) => c,
            None => return None,
        };

        if first_char == '"' {
            self.next_string()
        } else if first_char.is_numeric() {
            self.next_integer()
        } else if internal::can_word_start_with(first_char) {
            self.next_word()
        } else {
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

mod test
{
    use super::{Token,Tokenizer};

    macro_rules! expect_tokenize_into {
        // TODO: make the macro take several arguments
        // for several outputs
        ($input:expr => $( $output:expr ),* ) => {
            {
                let mut tokenizer = Tokenizer::new($input.chars());

                $(
                    assert_eq!(tokenizer.next().unwrap().unwrap(), $output);
                )*
            }
        }
    }

    #[test]
    fn test_string() {
        expect_tokenize_into!("\"hello\"" =>  Token::string("hello"));
        expect_tokenize_into!("\"hello abc\"" => Token::string("hello abc"));

        expect_tokenize_into!("\"hello world\"  \"it is me\"" =>
                              Token::string("hello world"),
                              Token::string("it is me"));
    }

    #[test]
    fn test_integer() {
        expect_tokenize_into!("123" => Token::integer(123));
        expect_tokenize_into!("0982" => Token::integer(982));
        expect_tokenize_into!("333 662" => Token::integer(333),
                                           Token::integer(662));
        expect_tokenize_into!("1 2 3 4" => Token::integer(1),
                                           Token::integer(2),
                                           Token::integer(3),
                                           Token::integer(4));
    }

    #[test]
    fn test_word() {
        expect_tokenize_into!("hello" => Token::word("hello"));
        expect_tokenize_into!("ab cd" => Token::word("ab"),
                                         Token::word("cd"));
        expect_tokenize_into!("a b c d" => Token::word("a"),
                                           Token::word("b"),
                                           Token::word("c"),
                                           Token::word("d"));
    }
}
