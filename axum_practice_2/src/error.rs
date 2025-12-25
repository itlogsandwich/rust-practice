use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum ApiError
{
    // NotFound,
    InvalidInput(String),
    InternalError(String),
}

impl IntoResponse for ApiError
{
    fn into_response(self) -> Response
    {
        let (status, error_message) = match self
        {
            // Self::NotFound => (StatusCode::NOT_FOUND, "Data not found".to_string()),
            Self::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!(
        {
            "error": error_message,

        }
        ));

        (status, body).into_response()
    }
}

impl From<IoError> for ApiError
{
    fn from(error:IoError) -> Self
    {
        ApiError::InternalError(error.to_string())
    }
}

impl From<String> for ApiError
{
    fn from(msg: String) -> Self
    {
        ApiError::InternalError(msg)
    }
}

impl From<serde_json::Error> for ApiError
{
    fn from(err: serde_json::Error) -> Self 
    {
        ApiError::InvalidInput(format!("JSON error: {err}"))
    }
}
impl core::fmt::Display for ApiError
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>
    {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for ApiError {}

