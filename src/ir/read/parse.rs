use super::{Tokenizer,Token,Resolve};

use {
    Global,Module,Value,Expression,Type,Block,
    Signature,Function,Parameter,Instruction,types,
    Unary, Binary, Condition, Register, Name,
};
use std;

pub type Result<T> = std::result::Result<T,String>;

pub const ENTRY_LABEL_NAME: &'static str = "entry";

/// An IR parser.
pub struct Parser<I: Iterator<Item=char>>
{
    tokenizer: Tokenizer<I>,

    module: Module,
    resolve: Resolve,
}

impl<I> Parser<I>
    where I: Iterator<Item=char>
{
    /// Creates a new parser.
    pub fn new(chars: I) -> Self {
        Parser {
            tokenizer: Tokenizer::new(chars),

            module: Module::empty(),
            resolve: Resolve::new(),
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

        self.module = self.resolve.resolve(self.module);
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

        let name = try!(self.parse_global_identifier());
        try!(self.expect(Token::equal_sign()));
        let value = try!(self.parse_value());

        try!(self.expect(Token::new_line()));

        let mut global = Global::new(name, value);
        self.resolve.give(&mut global);

        self.module.add_global(global);

        Ok(())
    }

    fn parse_function(&mut self) -> Result<()> {
        self.assert(keywords::function());

        self.resolve.begin_scope();

        let name = try!(self.parse_global_identifier());
        let params = try!(self.parse_parameter_list());
        let returns = try!(self.parse_function_returns());
        let body = try!(self.parse_body());

        let signature = Signature::new(params, returns);
        let mut function = Function::new(name, signature, body);

        self.resolve.end_scope();

        self.resolve.give(&mut function);
        self.module.add_function(function);

        Ok(())
    }

    /// Parses a curly-brace contained list of blocks.
    fn parse_body(&mut self) -> Result<Vec<Block>> {
        try!(self.eat_whitespace());
        try!(self.expect(Token::left_curly_brace()));
        try!(self.eat_whitespace());

        let mut blocks = Vec::new();

        let mut is_first_block = true;
        while try!(self.peek_something()) != Token::right_curly_brace() {

            let block = if is_first_block {
                is_first_block = false;
                try!(self.parse_entry_block())
            } else {
                try!(self.parse_block())
            };

            blocks.push(block);
        }

        try!(self.expect(Token::right_curly_brace()));

        Ok(blocks)
    }

    /// Parses a block which has an implied label of 'entry' if
    /// no label is specified.
    fn parse_entry_block(&mut self) -> Result<Block> {
        try!(self.eat_whitespace());

        let label = if try!(self.is_label_next()) {
            try!(self.parse_label())
        } else {
            ENTRY_LABEL_NAME.to_owned()
        };

        try!(self.eat_whitespace());

        let values = try!(self.parse_block_values());

        self.create_block(label, values)
    }

    fn parse_block(&mut self) -> Result<Block> {
        try!(self.eat_whitespace());
        let label = try!(self.parse_label());
        try!(self.eat_whitespace());

        let values = try!(self.parse_block_values());

        self.create_block(label, values)
    }

    fn parse_block_values(&mut self) -> Result<Vec<Value>> {
        let mut values = Vec::new();

        while !try!(self.is_end_of_block_next()) {
            let value = try!(self.parse_value());
            values.push(value);
            try!(self.eat_whitespace());
        }

        Ok(values)
    }

    fn create_block(&mut self, label: String, values: Vec<Value>) -> Result<Block> {
        let mut block = Block::new(label, values);
        self.resolve.give(&mut block);

        Ok(block)
    }

    fn is_label_next(&mut self) -> Result<bool> {
        let next = try!(self.peek_something());

        Ok(next == Token::colon())
    }

    fn is_end_of_block_next(&mut self) -> Result<bool> {
        let next = try!(self.peek_something());

        Ok(try!(self.is_label_next()) || next == Token::right_curly_brace())
    }

    fn parse_label(&mut self) -> Result<String> {
        try!(self.expect(Token::colon()));
        let name = try!(self.expect_word());
        try!(self.expect(Token::new_line()));

        Ok(name)
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<Parameter>> {
        try!(self.eat_whitespace());
        try!(self.expect(Token::left_parenthesis()));
        try!(self.eat_whitespace());

        let mut params = Vec::new();

        while try!(self.peek_something()) != Token::right_parenthesis() {
            try!(self.eat_whitespace());

            let name = try!(self.parse_local_identifier());
            try!(self.expect(Token::colon()));
            let ty = try!(self.parse_type());

            let mut param = Parameter::new(name, ty);
            self.resolve.give(&mut param);

            params.push(param);

            try!(self.maybe_eat(Token::comma()));
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

            try!(self.maybe_eat(Token::comma()));
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

    fn parse_condition(&mut self) -> Result<Condition> {
        let first_token = try!(self.peek_something());

        if first_token.is_boolean() {
            Ok(Condition::from_boolean(try!(self.parse_boolean())))
        } else {
            unimplemented!();
        }
    }

    fn parse_boolean(&mut self) -> Result<bool> {
        let value = try!(self.expect_word());

        match &*value {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(format!("{} is not a valid boolean value", value)),
        }
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
        let first_token = try!(self.peek_something());

        match first_token {
            Token::Word(..) => self.parse_word_expression(),
            Token::String(..) => self.parse_string_expression(),
            Token::Symbol(..) => self.parse_symbol_expression(),
            _ => Err(format!("unknown token for expression: {}", first_token)),
        }
    }

    fn parse_word_expression(&mut self)
        -> Result<Expression> {
        let first_word = try!(self.peek_word());

        if util::is_integer_type(&first_word) {
            self.parse_integer_expression()
        } else {
            self.parse_instruction()
        }
    }

    fn parse_integer_expression(&mut self)
        -> Result<Expression> {
        let type_word = self.assert_word();
        debug_assert!(type_word.starts_with('i') || type_word.starts_with('u'));

        let ty = try!(self.parse_integer_type(&type_word));
        let value = try!(self.expect_integer());

        Ok(Expression::integer(ty, value).unwrap())
    }

    fn parse_string_expression(&mut self)
        -> Result<Expression> {
        let string = self.assert_string();
        Ok(Expression::string(string))
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

    fn parse_global_identifier(&mut self) -> Result<String> {
        try!(self.expect(Token::at_sign()));
        self.expect_word()
    }

    fn parse_local_identifier(&mut self) -> Result<String> {
        try!(self.expect(Token::percent_sign()));
        self.expect_word()
    }

    fn parse_symbol_expression(&mut self) -> Result<Expression> {
        let symbol = self.assert_symbol();

        match &*symbol {
            "@" => {
                let name = try!(self.expect_word());
                self.parse_global_reference(name)
            },
            "%" => {
                let name = try!(self.expect_word());

                // check if this is a register assignment
                if try!(self.peek_something()) == Token::equal_sign() {
                    self.parse_register_assignment(name)
                } else {
                    self.parse_local_reference(name)
                }
            },
            _ => Err(format!("unknown expression: {}", symbol)),
        }
    }

    fn parse_register_assignment(&mut self, name: String)
        -> Result<Expression> {
        self.assert(Token::equal_sign());
        let value = try!(self.parse_value());

        let mut reg = Register::new(
            Name::named(name),
            value
        );

        self.resolve.give(&mut reg);

        Ok(reg.into())
    }

    // FIXME: this refers to something in the global scope, not
    // necessarily a global variable. come up with a better name.
    fn parse_global_reference(&mut self, name: String) -> Result<Expression> {
        Ok(self.resolve.reference(name))
    }

    fn parse_local_reference(&mut self, name: String) -> Result<Expression> {
        Ok(self.resolve.reference(name))
    }

    fn parse_instruction(&mut self)
        -> Result<Expression> {
        use instruction::*;

        let mnemonic = self.assert_word();

        match &*mnemonic {
            "add" => self.parse_binary_instruction::<Add>(),
            "sub" => self.parse_binary_instruction::<Sub>(),
            "mul" => self.parse_binary_instruction::<Mul>(),
            "div" => self.parse_binary_instruction::<Div>(),
            "shl" => self.parse_binary_instruction::<Shl>(),
            "shr" => self.parse_binary_instruction::<Shr>(),
            "call" => self.parse_unary_instruction::<Call>(),
            "ret" => self.parse_ret_instruction(),
            "br" => self.parse_br_instruction(),

            _ => Err(format!("unknown instruction: {}", mnemonic)),
        }
    }

    fn parse_unary_instruction<U>(&mut self) -> Result<Expression>
        where U: Unary {
        let op = try!(self.parse_value());

        Ok(U::with_operand(op).into())
    }

    fn parse_binary_instruction<B>(&mut self) -> Result<Expression>
        where B: Binary {
        let lhs = try!(self.parse_value());
        try!(self.expect(Token::comma()));
        let rhs = try!(self.parse_value());

        Ok(B::with_operands(lhs, rhs).into())
    }

    fn parse_ret_instruction(&mut self) -> Result<Expression> {
        use instruction::Return;

        let first_token = try!(self.peek_something());

        let target = if first_token.is_new_line() {
            None
        } else {
            Some(try!(self.parse_value()))
        };

        Ok(Return::new(target).into())
    }

    fn parse_br_instruction(&mut self) -> Result<Expression> {
        let condition = try!(self.parse_condition());
        try!(self.expect(Token::comma()));
        let target = try!(self.parse_value());

        Ok(Instruction::br(condition, target).into())
    }

    fn assert(&mut self, expected: Token) -> Token {
        self.expect(expected).unwrap()
    }

    fn expect_something(&mut self) -> Result<Token> {
        util::expect(self.tokenizer.next())
    }

    fn peek_something(&mut self) -> Result<Token> {
        util::expect(self.tokenizer.peek())
    }

    fn peek_word(&mut self) -> Result<String> {
        if let Token::Word(word) = try!(self.peek_something()) {
            Ok(word)
        } else {
            Err("expected word".to_owned())
        }
    }

    fn assert_word(&mut self) -> String { self.expect_word().unwrap() }
    fn assert_string(&mut self) -> String { self.expect_string().unwrap() }
    fn assert_symbol(&mut self) -> String { self.expect_symbol().unwrap() }

    fn expect_string(&mut self) -> Result<String> {
        if let Token::String(s) = try!(self.expect_something()) {
            Ok(s)
        } else {
            Err("expected a string".to_owned())
        }
    }

    fn expect_symbol(&mut self) -> Result<String> {
        if let Token::Symbol(s) = try!(self.expect_something()) {
            Ok(s)
        } else {
            Err("expected a symbol".to_owned())
        }
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
        assert!(!expected.is_empty());
        let token = try!(util::expect(self.tokenizer.next()));

        if expected.iter().any(|e| e==&token) {
            Ok(token)
        } else {
            if expected.len() == 1 {
                Err(format!("expected {} but got {}",
                            expected[0], token))
            } else { // multiple expected tokens
                Err(format!("expected one of {} but got {}",
                            ::util::comma_separated_values(expected.iter()),
                            token))
            }
        }
    }

    fn expect_word(&mut self) -> Result<String> {
        match self.expect_something() {
            Ok(token) => match token {
                Token::Word(w) => Ok(w.clone()),
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
        expect_global!("global @ABCD = i32 5" => "ABCD", Value::i32(5));

        expect_global!("global @hello_world = u127 38"
                       => "hello_world", Value::u(127, 38));

        expect_global!("global @ewf = i16 52" => "ewf", Value::i(16, 52));
    }
}

