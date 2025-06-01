use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;
use uuid;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("UUID parsing error: {0}")]
    UuidParseError(#[from] uuid::Error),

    #[error("Validation error: {0}")]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("Database error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Item not found")]
    NotFound,

    #[error("An internal server error occurred")]
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ValidationError(ref errors) => (
                StatusCode::BAD_REQUEST,
                format!("Input validation failed: {}", errors),
            ),
            AppError::UuidParseError(ref e) => {
                tracing::error!("UUID parsing error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error processing identifier.".to_string(),
                )
            }
            AppError::SqlxError(ref e) => {
                tracing::error!("SQLx error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal database error occurred.".to_string(),
                )
            }
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "The requested item was not found.".to_string(),
            ),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An unexpected error occurred.".to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
