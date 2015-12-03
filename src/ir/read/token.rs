
use util;
use std;
use std::error::Error;

pub type Result<T> = std::result::Result<T,String>;

/// A token.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Token
{
    /// A word.
    Word(String),
    /// A string literal.
    String(String),
    /// An integer literal.
    // TODO: use BigNum
    Integer(i64),
    /// A comment.
    ///
    /// If the comment is inline, it existed on the same line
    /// as the previous statement.
    ///
    /// For example
    ///
    /// ```
    /// add 2, 4 ; inline comment goes here
    /// ```
    Comment {
        inline: bool,
        text: String,
    },
    /// A symbol.
    Symbol(String),
    /// A new line.
    NewLine,
    /// End of file.
    /// Will always be yielded as the last
    /// token.
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

    pub fn comment<S>(text: S) -> Self
        where S: Into<String> {
        Token::Comment {
            inline: false,
            text: text.into(),
        }
    }

    pub fn inline_comment<S>(text: S) -> Self
        where S: Into<String> {
        Token::Comment {
            inline: true,
            text: text.into(),
        }
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

impl std::fmt::Display for Token
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Token::Word(ref w) => write!(fmt, "{}", w),
            &Token::String(ref s) => write!(fmt, "\"{}\"", s),
            &Token::Integer(ref i) => write!(fmt, "{}", i),
            &Token::Symbol(ref s) => write!(fmt, "{}", s),
            &Token::Comment { ref text, .. } => write!(fmt, " {}", text),
            &Token::NewLine => write!(fmt, "new line"),
            &Token::EOF => write!(fmt, "EOF"),
        }
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

    pub fn eat(&mut self) {
        self.next();
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

    finished: bool,
    preserve_comments: bool,
}

impl<I> Tokenizer<I>
    where I: Iterator<Item=char>
{
    pub fn new(chars: I) -> Self {
        Tokenizer {
            chars: Characters::new(chars),
            finished: false,
            preserve_comments: false,
        }
    }

    /// Allows the tokenizer to preserve comments it tokenizes.
    pub fn preserve_comments(mut self) -> Self {
        self.preserve_comments = true;
        self
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

            // If there are no tokens left, return EOF if
            // we haven't already, or None otherwise.
            None => {
                if self.finished {
                    return None;
                } else {
                    self.finished = true;
                    return Some(Ok(Token::eof()));
                }
            },
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
                                        Token::new_line(),
                                        Token::eof());

        expect_mapping!("\"hello abc\"" => Token::string("hello abc"),
                                           Token::new_line(),
                                           Token::eof());

        expect_mapping!("\"hello world\"  \"it is me\"" =>
                        Token::string("hello world"),
                        Token::string("it is me"),
                        Token::new_line(),
                        Token::eof());
    }

    #[test]
    fn test_integer() {
        expect_mapping!("123" => Token::integer(123),
                                 Token::new_line(),
                                 Token::eof());

        expect_mapping!("0982" => Token::integer(982),
                                  Token::new_line(),
                                  Token::eof());

        expect_mapping!("333 662" => Token::integer(333),
                                     Token::integer(662),
                                     Token::new_line(),
                                     Token::eof());

        expect_mapping!("1 2 3 4" => Token::integer(1),
                                     Token::integer(2),
                                     Token::integer(3),
                                     Token::integer(4),
                                     Token::new_line(),
                                     Token::eof());
    }

    #[test]
    fn test_word() {
        expect_mapping!("hello" => Token::word("hello"));

        expect_mapping!("ab cd" => Token::word("ab"),
                                   Token::word("cd"),
                                   Token::new_line(),
                                   Token::eof());

        expect_mapping!("a b c d" => Token::word("a"),
                                     Token::word("b"),
                                     Token::word("c"),
                                     Token::word("d"),
                                     Token::new_line(),
                                     Token::eof());
    }

    #[test]
    fn test_comments_not_preserved_by_default() {
        expect_mapping!("hello ; there" => Token::word("hello"),
                                           Token::new_line(),
                                           Token::eof());

        expect_mapping!(";why hello" => Token::new_line(),
                                        Token::eof());
    }

    #[test]
    fn test_comments() {
        expect_mapping_with!(Tokenizer::new("hello world ; this is me".chars())
                             .preserve_comments()
                             => Token::word("hello"),
                                Token::word("world"),
                                Token::comment(" this is me"),
                                Token::new_line(),
                                Token::eof());

        expect_mapping_with!(Tokenizer::new(";this is a test".chars())
                             .preserve_comments()
                             => Token::comment("this is a test"),
                                Token::new_line(),
                                Token::eof());
    }

    #[test]
    fn test_multiple() {
        expect_mapping!("12 bark \"earth\"" => Token::integer(12),
                                               Token::word("bark"),
                                               Token::string("earth"),
                                               Token::new_line(),
                                               Token::eof());

        expect_mapping!("a      23 gg \"a\"" => Token::word("a"),
                                                Token::integer(23),
                                                Token::word("gg"),
                                                Token::string("a"),
                                                Token::new_line(),
                                                Token::eof());
    }

    #[test]
    fn test_whitespace() {
        expect_mapping!("a\tb\t123\n\"qwer\"\n" => Token::word("a"),
                                                   Token::word("b"),
                                                   Token::integer(123),
                                                   Token::new_line(),
                                                   Token::string("qwer"),
                                                   Token::new_line(),
                                                   Token::new_line(),
                                                   Token::eof());
    }
}
