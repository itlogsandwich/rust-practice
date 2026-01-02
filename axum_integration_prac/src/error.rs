use axum::{Json, http::StatusCode};
use axum::response::{IntoResponse, Response};
use serde_json::json;
use std::io::Error as IoError;
// use std::fmt;
#[derive(Debug)]
pub enum Error
{
    InternalServer(String),
    // InvalidCredentials,
    // InvalidDeposit,
    // InvalidWithdrawal,
    // NotFound,
    // NotMatching,
    // PasswordLength,
}

// impl fmt::Display for Error
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
//     {
//         match self
//         {
//             Self::InternalServer(msg) => write!(f, "Internal Server Error"),
//
//             Self::InvalidCredentials => write!(f, "Invalid or Incorrect Credentials!"),
//
//             Self::InvalidDeposit => write!(f, "Invalid Deposit!"),
//             Self::InvalidWithdrawal => write!(f, "Invalid Withdrawal"),
//
//             Self::NotFound => write!(f, "Details not found!"),
//
//             Self::NotMatching => write!(f, "Passwords do not match"),
//             Self::PasswordLength => write!(f, "Password must contain at least 8 characters!"),
//
//         }
//     }
// }

impl IntoResponse for Error
{
    fn into_response(self) -> Response
    {
        let(status, error_message) = match self
        {
            Self::InternalServer(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),

        };

        let body = Json(json!(
        {
            "error": error_message,
        }
        ));

        (status, body).into_response()
    }
}

impl From<IoError> for Error
{
    fn from(error: IoError) -> Self
    {
        Error::InternalServer(error.to_string())
    }
}

impl From<String> for Error
{
    fn from(msg: String) -> Self
    {
        Error::InternalServer(msg)
    }
}

impl core::fmt::Display for Error
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>
    {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
