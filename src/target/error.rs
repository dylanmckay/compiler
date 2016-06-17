use std::io;

#[derive(Debug)]
pub enum Error
{
    IoError(io::Error),
    InvalidIR(String),
}

impl From<io::Error> for Error
{
    fn from(e: io::Error) -> Error {
        Error::IoError(e)
    }
}

