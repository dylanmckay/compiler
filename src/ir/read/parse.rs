use super::{Tokenizer,Token};

use ir;
use ir::{Module,Value,Expression};

/// An IR parser.
pub struct Parser<I: Iterator<Item=char>>
{
    tokenizer: Tokenizer<I>,
    module: Module,
}

impl<I> Parser<I>
    where I: Iterator<Item=char>
{
    /// Creates a new parser.
    pub fn new(chars: I) -> Self {
        Parser {
            tokenizer: Tokenizer::new(chars),
            module: Module::empty(),
        }
    }

    /// Gets parser which preserves comments.
    pub fn preserve_comments(mut self) -> Self {
        self.tokenizer = self.tokenizer.preserve_comments();
        self
    }

    /// Eats tokens while a predicate is true.
    pub fn parse(mut self) -> Result<Module,String> {
        while !self.tokenizer.is_finished() {
            try!(self.parse_next());
        }

        Ok(self.module)
    }

    fn parse_next(&mut self) -> Result<(),String> {
        try!(self.eat_new_lines());

        let first_token = match self.tokenizer.peek() {
            Some(result) => try!(result),
            None => return Ok(()),
        };

        if first_token == keywords::global() {
            self.parse_global()
        } else if first_token.is_eof() {
            Ok(())
        } else {
            Err(format!("unexpected token: {}", first_token))
        }
    }

    fn eat_new_lines(&mut self) -> Result<(),String> {
        self.tokenizer.eat_while(|token| token.is_new_line())
    }

    fn parse_global(&mut self) -> Result<(),String> {
        self.assert(keywords::global());

        let name = try!(self.expect_word());
        try!(self.expect(Token::equal_sign()));
        let value = try!(self.parse_value());

        try!(self.expect(Token::new_line()));

        self.module.add_global(ir::Global::new(name, value));
        Ok(())
    }

    fn parse_value(&mut self) -> Result<Value,String> {
        self.parse_expression().map(|expr| Value::new(expr))
    }

    fn parse_expression(&mut self) -> Result<Expression,String> {
        let first_token = try!(self.expect_something());

        match first_token {
            Token::Word(word) => self.parse_word_expression(word),
            _ => Err("unknown token for expression".into()),
        }
    }

    fn parse_word_expression(&mut self, first_word: String)
        -> Result<Expression,String> {
        if first_word.starts_with('i') || first_word.starts_with('u') {
            self.parse_integer_expression(first_word)
        } else {
            Err("unknown token for expression".into())
        }
    }

    fn parse_integer_expression(&mut self, type_word: String)
        -> Result<Expression,String> {
        debug_assert!(type_word.starts_with('i') || type_word.starts_with('u'));

        let ty = try!(self.parse_integer_type(type_word));
        let value = try!(self.expect_integer());

        Ok(Expression::integer(ty, value).unwrap())
    }

    fn parse_integer_type(&mut self, type_str: String)
        -> Result<ir::types::Integer,String> {
        let kind = match type_str.chars().next().unwrap() {
            'i' => ::util::IntegerKind::Signed,
            'u' => ::util::IntegerKind::Unsigned,
            _ => unreachable!(),
        };

        let width_str: String = type_str.chars().skip(1).collect();
        let width = try!(util::parse_integer(&width_str, 10));

        Ok(ir::types::Integer::new(kind, width as u16))
    }

    fn assert(&mut self, expected: Token) {
        self.expect(expected).unwrap();
    }

    fn expect_something(&mut self) -> Result<Token,String> {
        util::expect(self.tokenizer.next())
    }

    fn expect(&mut self, expected: Token) -> Result<Token,String> {
        // TODO: this could be a more descriptive error message
        self.expect_one_of(&[expected])
    }

    fn expect_one_of(&mut self, expected: &[Token]) -> Result<Token,String> {
        let token = try!(util::expect(self.tokenizer.next()));

        if expected.iter().any(|e| e==&token) {
            Ok(token)
        } else {
            Err(format!("expected one of {} but got {}",
                        ::util::comma_separated_values(expected.iter()),
                        token))
        }
    }

    fn expect_word(&mut self) -> Result<String,String> {
        match self.expect_something() {
            Ok(ref token) => match *token {
                Token::Word(ref w) => Ok(w.clone()),
                _ => Err("expected a word".into()),
            },
            Err(e) => Err(e),
        }
    }

    fn expect_integer(&mut self) -> Result<i64,String> {
        match self.expect_something() {
            Ok(ref token) => match *token {
                Token::Integer(i) => Ok(i),
                _ => Err("expected an integer".into()),
            },
            Err(e) => Err(e),
        }
    }
}

pub mod util
{
    /// Expects that an `Option<Result>` is `Some`.
    // TODO: give this a name that doesn't clash with Parser::expect
    pub fn expect<T>(val: Option<Result<T,String>>) -> Result<T,String> {
        match val {
            Some(result) => result,
            None => Err("expected a token".into()),
        }
    }

    pub fn parse_integer(string: &str, radix: u32) -> Result<i64,String> {
        match i64::from_str_radix(string, radix) {
            Ok(i) => Ok(i),
            Err(..) => Err("could not parse integer".into()),
        }
    }
}

pub mod keywords
{
    use ir::read::Token;

    pub fn global() -> Token {
        Token::word("global")
    }
}

#[allow(unused_imports)]
mod test
{
    use super::Parser;
    use ir::Value;

    #[cfg(test)]
    fn parse(text: &str) -> ::ir::Module {
        Parser::new(text.chars()).parse().expect("parsing failed")
    }

    macro_rules! expect_global {
        ($input:expr => $name:expr, $value:expr) => {
            {
                let module = parse($input);

                let global = module.globals().next().unwrap();
                assert_eq!(global.name(), $name);
                assert_eq!(global.value(), &$value);
            }
        }
    }

    #[test]
    fn globals() {
        expect_global!("global ABCD = i32 5" => "ABCD", Value::i32(5));

        expect_global!("global hello_world = u127 38"
                       => "hello_world", Value::u(127, 38));

        expect_global!("global ewf = i16 52" => "ewf", Value::i(16, 52));
    }
}

