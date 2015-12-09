use super::{Tokenizer,Token};

use {
    Global,Module,Value,Expression,Type,Block,
    Signature,Function,Parameter,types
};
use std;

pub type Result<T> = std::result::Result<T,String>;

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
    pub fn parse(mut self) -> Result<Module> {
        while self.tokenizer.peek().is_some() {
            try!(self.parse_next());
        }

        Ok(self.module)
    }

    fn parse_next(&mut self) -> Result<()> {
        try!(self.eat_new_lines());

        let first_token = match self.tokenizer.peek() {
            Some(result) => try!(result),
            None => return Ok(()),
        };

        if first_token == keywords::global() {
            self.parse_global()
        } else if first_token == keywords::function() {
            self.parse_function()
        } else {
            Err(format!("unexpected token: {}", first_token))
        }
    }

    fn eat_new_lines(&mut self) -> Result<()> {
        self.tokenizer.eat_while(|token| token.is_new_line())
    }

    fn parse_global(&mut self) -> Result<()> {
        self.assert(keywords::global());

        let name = try!(self.expect_word());
        try!(self.expect(Token::equal_sign()));
        let value = try!(self.parse_value());

        try!(self.expect(Token::new_line()));

        self.module.add_global(Global::new(name, value));
        Ok(())
    }

    fn parse_function(&mut self) -> Result<()> {
        self.assert(keywords::function());

        let name = try!(self.expect_word());
        let params = try!(self.parse_parameter_list());
        let returns = try!(self.parse_function_returns());
        let body = try!(self.parse_body());

        let signature = Signature::new(params, returns);
        let function = Function::new(name, signature, body);

        self.module.add_function(function);
        Ok(())
    }

    /// Parses a curly-brace contained list of blocks.
    fn parse_body(&mut self) -> Result<Vec<Block>> {
        try!(self.eat_whitespace());
        try!(self.expect(Token::left_curly_brace()));
        try!(self.eat_whitespace());

        let mut blocks = Vec::new();

        while try!(self.peek_something()) != Token::right_curly_brace() {
            let block = try!(self.parse_block());
            blocks.push(block);
        }

        try!(self.expect(Token::right_curly_brace()));

        Ok(blocks)
    }

    fn parse_block(&mut self) -> Result<Block> {
        self.expect(Token::right_curly_brace());
        unimplemented!();
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<Parameter>> {
        try!(self.eat_whitespace());
        try!(self.expect(Token::left_parenthesis()));
        try!(self.eat_whitespace());

        let mut params = Vec::new();

        while try!(self.peek_something()) != Token::right_parenthesis() {
            try!(self.eat_whitespace());

            let name = try!(self.expect_word());
            try!(self.expect(Token::colon()));
            let ty = try!(self.parse_type());

            params.push(Parameter::new(name, ty));

            self.maybe_eat(Token::comma());
            try!(self.eat_whitespace());
        }

        self.assert(Token::right_parenthesis());

        Ok(params)
    }

    fn parse_type_list(&mut self) -> Result<Vec<Type>> {
        try!(self.eat_whitespace());

        let mut types = Vec::new();

        while try!(self.peek_something()) != Token::left_curly_brace() {
            try!(self.eat_whitespace());

            let ty = try!(self.parse_type());
            types.push(ty);

            self.maybe_eat(Token::comma());
            try!(self.eat_whitespace());
        }

        Ok(types)
    }

    fn parse_function_returns(&mut self) -> Result<Vec<Type>> {
        try!(self.eat_whitespace());
        let first_token = try!(self.peek_something());

        if first_token == Token::function_arrow() {
            self.assert(Token::function_arrow());
            try!(self.eat_whitespace());

            self.parse_type_list()
        } else if first_token == Token::left_curly_brace() {
            Ok(Vec::new())
        } else {
            Err(format!("expected -> or {{ but got {}", first_token))
        }
    }

    fn parse_value(&mut self) -> Result<Value> {
        self.parse_expression().map(|expr| Value::new(expr))
    }

    fn parse_type(&mut self) -> Result<Type> {
        let first_token =  try!(self.expect_something());

        match first_token {
            Token::Word(first_word) => self.parse_word_type(first_word),
            _ => Err(format!("unknown token for type: {}",
                             first_token)),
        }
    }

    fn parse_word_type(&mut self, first_word: String)
        -> Result<Type> {
        if util::is_integer_type(&first_word) {
            self.parse_integer_type(&first_word).map(|t| t.into())
        } else {
            Err(format!("unknown type: {}", first_word))
        }
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        let first_token = try!(self.expect_something());

        match first_token {
            Token::Word(word) => self.parse_word_expression(word),
            Token::String(string) => self.parse_string_expression(string),
            _ => Err("unknown token for expression".into()),
        }
    }

    fn parse_word_expression(&mut self, first_word: String)
        -> Result<Expression> {
        if util::is_integer_type(&first_word) {
            self.parse_integer_expression(&first_word)
        } else {
            Err("unknown token for expression".into())
        }
    }

    fn parse_integer_expression(&mut self, type_word: &str)
        -> Result<Expression> {
        debug_assert!(type_word.starts_with('i') || type_word.starts_with('u'));

        let ty = try!(self.parse_integer_type(type_word));
        let value = try!(self.expect_integer());

        Ok(Expression::integer(ty, value).unwrap())
    }

    fn parse_string_expression(&mut self, string: String)
        -> Result<Expression> {
        unimplemented!();
    }

    fn parse_integer_type(&mut self, type_str: &str)
        -> Result<types::Integer> {
        let kind = match type_str.chars().next().unwrap() {
            'i' => ::util::IntegerKind::Signed,
            'u' => ::util::IntegerKind::Unsigned,
            _ => unreachable!(),
        };

        let width_str: String = type_str.chars().skip(1).collect();
        let width = try!(util::parse_integer(&width_str, 10));

        Ok(types::Integer::new(kind, width as u16))
    }

    fn assert(&mut self, expected: Token) {
        self.expect(expected).unwrap();
    }

    fn expect_something(&mut self) -> Result<Token> {
        util::expect(self.tokenizer.next())
    }

    fn peek_something(&mut self) -> Result<Token> {
        util::expect(self.tokenizer.peek())
    }

    fn maybe_eat(&mut self, token: Token) -> Result<()> {
        match self.tokenizer.peek() {
            Some(Ok(peeked)) => {
                if peeked == token {
                    self.tokenizer.eat();
                }

                Ok(())
            },
            Some(Err(e)) => Err(e),
            None => Ok(()),
        }
    }

    fn expect(&mut self, expected: Token) -> Result<Token> {
        // TODO: this could be a more descriptive error message
        self.expect_one_of(&[expected])
    }

    fn expect_one_of(&mut self, expected: &[Token]) -> Result<Token> {
        let token = try!(util::expect(self.tokenizer.next()));

        if expected.iter().any(|e| e==&token) {
            Ok(token)
        } else {
            Err(format!("expected one of {} but got {}",
                        ::util::comma_separated_values(expected.iter()),
                        token))
        }
    }

    fn expect_word(&mut self) -> Result<String> {
        match self.expect_something() {
            Ok(ref token) => match *token {
                Token::Word(ref w) => Ok(w.clone()),
                _ => Err("expected a word".into()),
            },
            Err(e) => Err(e),
        }
    }

    fn expect_integer(&mut self) -> Result<i64> {
        match self.expect_something() {
            Ok(ref token) => match *token {
                Token::Integer(i) => Ok(i),
                _ => Err("expected an integer".into()),
            },
            Err(e) => Err(e),
        }
    }

    fn eat_whitespace(&mut self) -> Result<()> {
        self.tokenizer.eat_while(|t| t.is_new_line())
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

    pub fn is_integer_type(word: &str) -> bool {
        if word.starts_with('i') || word.starts_with('u') {
            let next_part: String = word.chars().skip(1).collect();

            is_integer(&next_part, 10)
        } else {
            false
        }
    }

    pub fn is_integer(string: &str, radix: u32) -> bool {
        parse_integer(string, radix).is_ok()
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
    use read::Token;

    pub fn global() -> Token {
        Token::word("global")
    }

    pub fn function() -> Token {
        Token::word("fn")
    }
}

#[allow(unused_imports)]
mod test
{
    use super::Parser;
    use {Value,Function,Signature};

    #[cfg(test)]
    fn parse(text: &str) -> ::Module {
        Parser::new(text.chars()).parse().expect("parsing failed")
    }

    macro_rules! expect_global {
        ($input:expr => $name:expr, $value:expr) => {
            {
                let module = parse($input);

                let global = module.globals()
                                   .next()
                                   .expect("no globals were parsed");

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

