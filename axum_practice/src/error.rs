use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::From;
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, From, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error
{
    #[from(String, &str, &String)]
    Custom(String),

    LoginFail,
    
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,

    TicketDeleteFailIdNotFound {id: u64},
}

impl Error
{
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError)
    {
        #[allow(unreachable_patterns)]
        match self
        {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            
            Self::AuthFailNoAuthTokenCookie | Self::AuthFailTokenWrongFormat | Self::AuthFailCtxNotInRequestExt =>
            {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            },
 
            Self::TicketDeleteFailIdNotFound { .. } =>
            {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            },

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}
impl IntoResponse for Error
{
    fn into_response(self) -> Response 
    {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response

    }
}


impl core::fmt::Display for Error
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>
    {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error{}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError
{
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
