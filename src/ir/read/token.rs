
use std;

/// A token.
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
}

impl<I> Characters<I>
    where I: Iterator<Item=char>
{
    pub fn new(it: I) -> Self {
        Characters {
            it: it.peekable(),
            finished: false,
        }
    }
}

impl<I> Iterator for Characters<I>
    where I: Iterator<Item=char>
{
    type Item = char;

    fn next(&mut self) -> Option<char> {
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
    chars: std::iter::Peekable<Characters<I>>,
}

impl<I> Tokenizer<I>
    where I: Iterator<Item=char>
{
    pub fn new(chars: I) -> Self {
        Tokenizer {
            chars: Characters::new(chars).peekable(),
        }
    }

    fn next_string(&mut self) -> Option<Token> {
        unimplemented!();
    }

    fn next_symbol(&mut self) -> Option<Token> {
        unimplemented!();
    }
}

impl<I> Iterator for Tokenizer<I>
    where I: Iterator<Item=char>
{
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let first_char = match self.chars.peek() {
            Some(&c) => c,
            None => return None,
        };

        if first_char == '"' {
            self.next_string()
        } else {
            self.next_symbol()
        }
    }
}
