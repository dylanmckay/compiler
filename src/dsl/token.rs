
use std::error::Error;
use std::fmt;

// TODO: float support is broken

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Keyword
{
    Class,
}

impl Keyword {
    pub fn parse(word: &str) -> Option<Self> {
        match word {
            "class" => Some(Keyword::Class),
            _ => None,
        }
    }
}

impl fmt::Display for Keyword
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        match *self {
            Keyword::Class => "class",
        }.fmt(fmt)
    }
}

/// A token.
#[derive(PartialEq,Debug)]
pub enum Token
{
    Keyword(Keyword),
    Identifier(String),
    String(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    Symbol(&'static str),
}

pub struct Tokenizer<I: Iterator<Item=Result<char,String>>>
{
    chars: I,

    /// A temporary peek buffer.
    /// Contains `\0` if it is empty.
    peek_buf: char,
}

impl<I> Tokenizer<I>
    where I: Iterator<Item=Result<char,String>>
{
    pub fn new(chars: I) -> Self {
        Tokenizer {
            chars: chars,
            peek_buf: '\0',
        }
    }

    pub fn tokenize(&mut self) -> Option<Result<Token,String>> {
        // eat whitespace
        match self.eat_whitespace() {
            Ok(..) => (),
            Err(e) => { return Some(Err(e)); },
        }

        let peeked = match util::expect(self.peek_char()) {
            Ok(a) => a,
            Err(a) => { return Some(Err(a)); },
        };

        if peeked.is_digit(10) || peeked == '.' {
            Some(self.tokenize_numeric_literal())
        } else if peeked == '"' {
            Some(self.tokenize_string_literal())
        } else if peeked == '_' || peeked.is_alphabetic() {
            Some(self.tokenize_word())
        } else { // resort to token
            Some(self.tokenize_symbol())
        }
    }

    fn tokenize_numeric_literal(&mut self) -> Result<Token,String> {

        let mut chars = String::new();
        let mut base = 10;
        
        let first_char = try!(util::expect(self.next_char()));

        // check prefix
        if first_char == '0' {
            let prefix = try!(util::expect(self.peek_char()));

            if prefix == 'b' { // binary
                base = 2;
                try!(self.eat_char());
            } else if prefix == 'd' { // decimal
                base = 10;
                try!(self.eat_char());
            } else if prefix == 'x' { // hex
                base = 16;
                try!(self.eat_char());
            } else { // no base specifief, just a normal literal
                chars.push(first_char);
            }
        } else {
            chars.push(first_char); // handle normally
        }

        loop {
            let c = match self.peek_char() {
                Some(Ok(c)) => c,
                Some(Err(e)) => { return Err(e); },
                None => { break; },
            };

            if c.is_digit(base) || c == '.' {
                try!(self.eat_char());
                chars.push(c);
            } else {
                break;
            }
        }

        if chars.contains('.') { // a float
            match chars.parse() {
                Ok(c) => Ok(Token::FloatLiteral(c)),
                Err(e) => Err(e.description().into()),
            }    
        } else { // an integer
            match i64::from_str_radix(&chars, base) {
                Ok(c) => Ok(Token::IntegerLiteral(c)),
                Err(e) => Err(e.description().into()),
            }
        }
    }

    fn tokenize_string_literal(&mut self) -> Result<Token,String> {
        assert!(self.next_char() == Some(Ok('"')));

        let mut result = String::new();

        loop {
            let c = try!(util::expect(self.next_char()));

            if c == '"' {
                break;
            } else {
                result.push(c);
            }
        }

        Ok(Token::String(result))
    }

    fn tokenize_word(&mut self) -> Result<Token,String> {
        let mut word = String::new();

        // push the first character
        word.push(try!(util::expect(self.next_char())));

        loop {
            let c = match self.peek_char() {
                Some(Ok(c)) => c,
                Some(Err(e)) => { return Err(e); },
                None => { break; },
            };

            if (c.is_alphanumeric() || c == '_') &&
               !c.is_whitespace() {

                word.push(c);
                try!(self.eat_char());
            } else {
                break;
            }
        }

        Ok(match Keyword::parse(&word) {
            Some(key) => Token::Keyword(key),
            None => Token::Identifier(word)
        })
    }

    fn tokenize_symbol(&mut self) -> Result<Token,String> {
        let first_char = try!(util::expect(self.next_char()));
        let second_char = try!(util::expect(self.peek_char()));

        let sym = match (first_char,second_char) {
            ('+', '=') => "+=",
            ('-', '=') => "-=",
            ('*', '=') => "*=",
            ('/', '=') => "/=",
            ('&', '&') => "&&",
            ('|', '|') => "||",
            ('&', _) => "&",
            ('|', _) => "|",
            ('+', _) => "+",
            ('-', _) => "-",
            ('*', _) => "*",
            ('/', _) => "/",
            (';', _) => ";",
            ('{', _) => "{",
            ('}', _) => "}",
            ('[', _) => "[",
            (']', _) => "]",
            ('^', _) => "]",
            (c, _) => { return Err(format!("unexpected token {}", c)); },

        };

        Ok(Token::Symbol(sym))

    }

    /// Gets the next character.
    /// Returns `Ok('\0')` if there are no more characters.
    fn next_char(&mut self) -> Option<Result<char,String>> {

        if self.peek_buf != '\0' {
            let c = self.peek_buf;
            self.peek_buf = '\0';

            Some(Ok(c))
        } else {
            self.chars.next()
        }
    }

    fn peek_char(&mut self) -> Option<Result<char, String>> {
        let c = match self.next_char() {
            Some(Ok(c)) => c,
            Some(Err(e)) => { return Some(Err(e)); },
            None => { return None; },
        };

        // it would've been comsumed if it wasn't empty
        assert!(self.peek_buf == '\0');

        self.peek_buf = c;
        Some(Ok(c))
    }

    fn eat_char(&mut self) -> Result<(),String> {
        match self.next_char() {
            None | Some(Ok(..)) => Ok(()),
            Some(Err(e)) => Err(e),
        }
    }

    fn eat_whitespace(&mut self) -> Result<(),String> {
        loop {
            if let Some(p) = self.peek_char() {
                let peeked = try!(p);


                if peeked == '\0' || // End of line
                    !peeked.is_whitespace() { // or we encountered a non-white char
                    break;
                } else { // whitespace
                    try!(self.eat_char());
                }
            }
        }

        Ok(())
    }
}

impl<I: Iterator<Item=Result<char,String>>> Iterator for Tokenizer<I>
{
    type Item = Result<Token,String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokenize()
    }
}


mod util
{
    pub fn expect_or<S,E>(val: Option<Result<S,E>>, or: E) -> Result<S,E> {
        match val {
            Some(val) => val,
            None => Err(or),
        } 
    }

    pub fn expect<S>(val: Option<Result<S,String>>) -> Result<S,String> {
        self::expect_or(val, "expected a value".into())
    }
}

#[test]
fn test_int() {
    let s = "123 321 0d2  0xff 0b11001";
    let mut tokenizer = Tokenizer::new(s.chars().map(|a| Ok(a)));

    assert_eq!(tokenizer.tokenize().unwrap().unwrap(), Token::IntegerLiteral(123));
    assert_eq!(tokenizer.tokenize().unwrap().unwrap(), Token::IntegerLiteral(321));
    assert_eq!(tokenizer.tokenize().unwrap().unwrap(), Token::IntegerLiteral(2));
    assert_eq!(tokenizer.tokenize().unwrap().unwrap(), Token::IntegerLiteral(0xff));
    assert_eq!(tokenizer.tokenize().unwrap().unwrap(), Token::IntegerLiteral(0b11001));
}

#[test]
fn test_float() {
/*    let s = "1.9";
    let mut tokenizer = Tokenizer::new(s.chars().map(|a| Ok(a)));

    tokenizer.tokenize().unwrap().unwrap();
    tokenizer.tokenize().unwrap().unwrap();*/
}

#[test]
fn test_str() {
    let s = " \"Hello World!\" \" my name is jack\"";
    let mut tokenizer = Tokenizer::new(s.chars().map(|a| Ok(a)));

    assert_eq!(tokenizer.tokenize().unwrap().unwrap(), Token::String("Hello World!".into()));
    assert_eq!(tokenizer.tokenize().unwrap().unwrap(), Token::String(" my name is jack".into()));
}

#[test]
fn test_keywords() {
    let s = "class class 12 class";
    let mut tokenizer = Tokenizer::new(s.chars().map(|a| Ok(a)));

    assert_eq!(tokenizer.tokenize().unwrap().unwrap(), Token::Keyword(Keyword::Class));
    assert_eq!(tokenizer.tokenize().unwrap().unwrap(), Token::Keyword(Keyword::Class));
    assert_eq!(tokenizer.tokenize().unwrap().unwrap(), Token::IntegerLiteral(12));
    assert_eq!(tokenizer.tokenize().unwrap().unwrap(), Token::Keyword(Keyword::Class));
}
