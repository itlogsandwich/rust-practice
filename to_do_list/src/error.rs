use std::fmt;
use std::num::ParseIntError;
use std::io::Error as IOErr;
use serde_json::Error as JsonError;
#[derive(Debug)]
pub enum Error
{
    ParsingError(String),

    IOError(String),
    SerdeError(String),
    
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
            Error::ParsingError(msg) => write!(f, "Parsing failed! {msg}"),
            
            Error::IOError(msg) => write!(f, "IO Error has occured! {msg}"),
            Error::SerdeError(msg)=> write!(f, "Conversion error! {msg}"),

            Error::InvalidIndex => write!(f, "Invalid Index!"),
            Error::AlreadyDone => write!(f, "Already marked as done!"),
            Error::NotFound => write!(f, "Task not found!"),
        }
    }
}

impl From<ParseIntError> for Error
{
    fn from(msg: ParseIntError) -> Self
    {
        Error::ParsingError(msg.to_string())
    }
}

impl From<IOErr> for Error
{
    fn from(msg: IOErr) -> Self
    {
        Error::IOError(msg.to_string())
    }
}

impl From<JsonError> for Error
{
    fn from(msg: JsonError) -> Self
    {
        Error::SerdeError(msg.to_string())
    }
}
impl std::error::Error for Error {}
