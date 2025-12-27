use std::fmt;

#[derive(Debug)]
pub enum Error
{
    Invalid,
    NotFound,
}

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Error::Invalid => write!(f, "Invalid!"),
            Error::NotFound => write!(f, "Details not found!"),
        }
    }
}

impl std::error::Error for Error {}
