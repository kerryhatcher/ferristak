//! Unified application error type.
//!
//! `AppError` is the single error surface for HTTP handlers. It converts to a
//! consistent JSON body and maps internal failures to appropriate status codes
//! without leaking implementation details to clients.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// Errors that can be returned from a request handler.
#[derive(Debug, Error)]
pub enum AppError {
    /// The request payload failed validation.
    #[error("{0}")]
    Validation(String),

    /// Missing or invalid credentials.
    #[error("authentication required")]
    Unauthorized,

    /// Authenticated but not permitted to perform the action.
    #[error("insufficient permissions")]
    Forbidden,

    /// The requested resource does not exist.
    #[error("resource not found")]
    NotFound,

    /// The request conflicts with the current state (e.g. duplicate email).
    #[error("{0}")]
    Conflict(String),

    /// An unexpected internal failure. Details are logged, not returned.
    #[error("internal server error")]
    Internal(#[from] anyhow::Error),
}

impl AppError {
    /// Construct a validation error from anything displayable.
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        if let sqlx::Error::Database(ref db_err) = err {
            if db_err.is_unique_violation() {
                return Self::Conflict("resource already exists".to_string());
            }
            if db_err.is_foreign_key_violation() {
                return Self::Validation("referenced resource does not exist".to_string());
            }
        }
        Self::Internal(anyhow::Error::new(err))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            Self::Validation(_) => (StatusCode::UNPROCESSABLE_ENTITY, "validation_error"),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            Self::Forbidden => (StatusCode::FORBIDDEN, "forbidden"),
            Self::NotFound => (StatusCode::NOT_FOUND, "not_found"),
            Self::Conflict(_) => (StatusCode::CONFLICT, "conflict"),
            Self::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error"),
        };

        if status.is_server_error() {
            tracing::error!(error = %self, "request failed");
        } else {
            tracing::debug!(error = %self, "request rejected");
        }

        let body = Json(json!({
            "error": {
                "code": code,
                "message": self.to_string(),
            }
        }));

        (status, body).into_response()
    }
}
