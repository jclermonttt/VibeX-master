use axum::{
    http::StatusCode,
    response::{IntoResponse, Response as AxumResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> Response<T> {
    pub fn new<M: Into<String>>(message: M, data: Option<T>) -> Self {
        Response {
            message: Some(message.into()),
            data,
        }
    }
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> AxumResponse {
        match serde_json::to_string(&self) {
            Ok(body) => AxumResponse::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(body.into())
                .unwrap(),
            Err(_) => AxumResponse::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Internal Server Error".into())
                .unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl ErrorResponse {
    pub fn new<M: Into<String>>(message: M) -> Self {
        ErrorResponse {
            message: message.into(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> AxumResponse {
        let body = match serde_json::to_string(&self) {
            Ok(body) => body,
            Err(_) => "Failed to serialize error message".to_string(),
        };

        AxumResponse::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(body.into())
            .unwrap()
    }
}
