
use util;
use std;

pub type Result<T> = std::result::Result<T,String>;

/// A token.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Token
{
    Word(String),

    StringLiteral(String),
    // TODO: use BigNum
    IntegerLiteral(i64),

    Symbol(String),

    NewLine,
    EOF,
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

        Some(Ok(Token::StringLiteral(string.collect())))
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
        } else {
            self.next_symbol()
        }
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
        expect_tokenize_into!("\"hello\"" =>  Token::StringLiteral("hello".into()));
        expect_tokenize_into!("\"hello abc\"" => Token::StringLiteral("hello abc".into()));

        expect_tokenize_into!("\"hello world\"  \"it is me\"" =>
                              Token::StringLiteral("hello world".into()),
                              Token::StringLiteral("it is me".into()));
    }
}
