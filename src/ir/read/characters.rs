use std;

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

