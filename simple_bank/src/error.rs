use std::fmt;

#[derive(Debug)]
pub enum Error
{
    InvalidCredentials,
    InvalidDeposit,
    InvalidWithdrawal,
    NotFound,
    NotMatching,
    PasswordLength,
}

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Error::InvalidCredentials => write!(f, "Invalid or Incorrect Credentials!"),

            Error::InvalidDeposit => write!(f, "Invalid Deposit!"),
            Error::InvalidWithdrawal => write!(f, "Invalid Withdrawal"),

            Error::NotFound => write!(f, "Details not found!"),

            Error::NotMatching => write!(f, "Passwords do not match"),
            Error::PasswordLength => write!(f, "Password must contain at least 8 characters!"),

        }
    }
}

impl std::error::Error for Error {}
