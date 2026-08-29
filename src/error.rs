use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Resource {resource_type} not found for '{resource_id}'")]
    ResourceNotFound {
        resource_type: String,
        resource_id: String,
    },
    #[error("Login failed at {provider} (code {status_code}): {message}")]
    LoginFailed {
        provider: String,
        status_code: u16,
        message: String,
    },
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("Internal Server Error: {0}")]
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, type_, message) = match &self {
            AppError::ResourceNotFound { .. } => {
                (StatusCode::NOT_FOUND, "notFound", self.to_string())
            }
            AppError::LoginFailed { .. } => {
                (StatusCode::NOT_FOUND, "loginFailed", self.to_string())
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internalError",
                self.to_string(),
            ),
        };

        let body = Json(json!({
            "type": type_,
            "message": message,
        }));

        (status, body).into_response()
    }
}
