use std;

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
    /// ``` ir
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
}

impl Token
{
    pub fn comma() -> Self { Token::symbol(",") }
    pub fn colon() -> Self { Token::symbol(":") }
    pub fn left_parenthesis() -> Self { Token::symbol("(") }
    pub fn right_parenthesis() -> Self { Token::symbol(")") }
    pub fn at_sign() -> Self { Token::symbol("@") }
    pub fn percent_sign() -> Self { Token::symbol("%") }
    pub fn left_curly_brace() -> Self { Token::symbol("{") }
    pub fn right_curly_brace() -> Self { Token::symbol("}") }
    pub fn equal_sign() -> Self { Token::symbol("=") }
    pub fn function_arrow() -> Self { Token::symbol("->") }

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

    pub fn is_word(&self) -> bool {
        if let Token::Word(..) = *self { true } else { false }
    }

    pub fn is_string(&self) -> bool {
        if let Token::String(..) = *self { true } else { false }
    }

    pub fn is_integer(&self) -> bool {
        if let Token::String(..) = *self { true } else { false }
    }

    pub fn is_symbol(&self) -> bool {
        if let Token::Symbol(..) = *self { true } else { false }
    }

    pub fn is_comment(&self) -> bool {
        if let Token::Comment { .. } = *self { true } else { false }
    }

    pub fn is_new_line(&self) -> bool {
        if let Token::NewLine = *self { true } else { false }
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
        }
    }
}

