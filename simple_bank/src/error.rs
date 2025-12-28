use std::fmt;

#[derive(Debug)]
pub enum Error
{
    InvalidName,
    InvalidDeposit,
    NotFound,
    AlreadyExists,
}

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Error::InvalidName => write!(f, "Invalid Name!"),
            Error::InvalidDeposit => write!(f, "Invalid Deposit!"),
            Error::NotFound => write!(f, "Details not found!"),
            Error::AlreadyExists => write!(f, "Account alraedy exists"),
        }
    }
}

impl std::error::Error for Error {}
