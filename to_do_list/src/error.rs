use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error
{
    ParsingError,

    InvalidIndex,
    AlreadyDone,
    NotFound,


}

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Error::ParsingError => write!(f, "Parsing failed!"),

            Error::InvalidIndex => write!(f, "Invalid Index!"),
            Error::AlreadyDone => write!(f, "Already marked as done!"),
            Error::NotFound => write!(f, "Task not found!"),
        }
    }
}

impl From<ParseIntError> for Error
{
    fn from(_: ParseIntError) -> Self
    {
        Error::ParsingError
    }
}

impl std::error::Error for Error {}
