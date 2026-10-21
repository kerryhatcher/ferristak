//! `AuthUser` — an axum extractor for authenticated endpoints.
//!
//! Reads `Authorization: Bearer <jwt>`, verifies it, and exposes the caller's
//! identity. Any problem yields `401 Unauthorized`.

use axum::{extract::FromRequestParts, http::request::Parts};
use uuid::Uuid;

use crate::{auth::jwt::Claims, error::AppError, state::AppState};

/// The authenticated caller, derived from the request's bearer token.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub role: String,
}

impl AuthUser {
    /// Whether the caller holds the `admin` role.
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let token = header
            .strip_prefix("Bearer ")
            .or_else(|| header.strip_prefix("bearer "))
            .map(str::trim)
            .filter(|token| !token.is_empty())
            .ok_or(AppError::Unauthorized)?;

        let Claims {
            sub, email, role, ..
        } = state.jwt.verify(token)?;

        let id = Uuid::parse_str(&sub).map_err(|_| AppError::Unauthorized)?;

        Ok(Self { id, email, role })
    }
}
