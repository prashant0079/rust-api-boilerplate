use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
// convenience for later
pub type Result<T> = core::result::Result<T, Error>;

//Key Idea here: NEVER send internal details about the error to the client

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag="type", content="data")]
pub enum Error {
    LoginFail,

    // -- Auth Errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExtension,

    // -- Model erros.
    TicketDeleteFailIdNotFound { id: u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let mut response = (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response();

        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Error::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // Auth
            Error::AuthFailCtxNotInRequestExtension
            | Error::AuthFailNoAuthTokenCookie
            | Error:: AuthFailTokenWrongFormat => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }

            // -- Fallback
            #[allow(unreachable_patterns)]
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}