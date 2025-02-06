use axum::http::StatusCode;
use axum::response::IntoResponse;
// use dotenvy::Error as DotEnvError;
use log::ParseLevelError;
use std::{env::VarError, ffi::OsString, io, num::ParseIntError, string::FromUtf8Error};
use thiserror::Error;

/// ## ServerError
#[derive(Error, Debug)]
pub enum ServerError {
    #[error(transparent)]
    Axum(#[from] axum::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error(transparent)]
    StdVar(#[from] VarError),

    #[error(transparent)]
    Io(#[from] io::Error),

    // #[error(transparent)]
    // DotEnv(#[from] DotEnvError),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),

    #[error(transparent)]
    FromUtf8(#[from] FromUtf8Error),

    #[error(transparent)]
    ParseLevel(#[from] ParseLevelError),

    #[error("{0:?}")]
    Os(OsString),
}

impl ServerError {
    pub fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl From<OsString> for ServerError {
    fn from(value: OsString) -> Self {
        ServerError::Os(value)
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        log::error!("{self:#?}");

        (self.status_code(), self).into_response()
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Invalid system id")]
    InvalidSystemId,

    #[error("Invalid input id")]
    InputInvalid,

    #[error("Input does not exist")]
    InputDoesNotExist,

    #[error("Metrics error")]
    MetricsError,

    #[error("Unauthorized")]
    Unauthorized,

    #[error(transparent)]
    Server(ServerError),
}

impl<T> From<T> for Error
where
    T: Into<ServerError>,
{
    fn from(value: T) -> Self {
        Error::Server(value.into())
    }
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Error::UserAlreadyExists => StatusCode::CONFLICT,
            Error::InvalidSystemId => StatusCode::BAD_REQUEST,
            Error::InputInvalid => StatusCode::BAD_REQUEST,
            Error::InputDoesNotExist => StatusCode::NOT_FOUND,
            Error::MetricsError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::Server(err) => err.status_code(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        log::error!("{self:#?}");

        match self {
            Error::Server(err) => err.into_response(),
            err => {
                let status_code = err.status_code();
                (status_code, err.to_string()).into_response()
            }
        }
    }
}
