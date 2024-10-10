use axum::{http::StatusCode, response::{IntoResponse, Response}};
use derive_more::derive::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    RepoErr(crate::repository::error::Error),
}

impl core::fmt::Display for Error {
    fn fmt(
        &self,
        fmt: &mut core::fmt::Formatter<'_>,
    ) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            Error::RepoErr(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Repository error: {}", err)),
            Error::SeaErr(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", err)),
        };

        let body = serde_json::json!({ "error": body });
        Response::builder()
            .status(status)
            .header("Content-Type", "application/json")
            .body(body.to_string().into())
            .unwrap()
    }
}