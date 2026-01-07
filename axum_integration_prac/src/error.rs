use axum::{Json, http::StatusCode, http::Error as AxumError};
use axum::response::{IntoResponse, Response};
use serde_json::json;
use std::io::Error as IoError;
#[derive(Debug)]
pub enum Error
{
    InternalServer(String),
    InvalidCredentials,
    InvalidDeposit,
    InvalidWithdrawal,
    NotFound,
    NotMatching,
    PasswordLength,
}

impl IntoResponse for Error
{
    fn into_response(self) -> Response
    {
        let(status, error_message) = match self
        {
            Self::InternalServer(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),

            Self::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Wrong PIN".to_string()),

            Self::InvalidDeposit => (StatusCode::BAD_REQUEST, "Fail to Deposit!".to_string()),
            Self::InvalidWithdrawal => (StatusCode::BAD_REQUEST, "Insufficient Funds!".to_string()),

            Self::NotFound => (StatusCode::NOT_FOUND, "Account not Found".to_string()),
            Self::NotMatching => (StatusCode::BAD_REQUEST, "Passwords do not match".to_string()),
            Self::PasswordLength => (StatusCode::BAD_REQUEST, "Password must contain at least 8 characters!".to_string()),             
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

impl From<AxumError> for Error
{
    fn from(error: AxumError) -> Self
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

