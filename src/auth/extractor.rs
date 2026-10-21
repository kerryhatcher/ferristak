//! `AuthUser` — an axum extractor for authenticated endpoints.
//!
//! Reads `Authorization: Bearer <jwt>`, verifies it, and exposes the caller's
//! identity. Any problem yields `401 Unauthorized`.

use axum::{extract::FromRequestParts, http::request::Parts};
use uuid::Uuid;

use crate::{config::EventVisibility, error::AppError, state::AppState};

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

    /// Owner filter to apply when reading CoT events. `None` means every owner
    /// is visible; `Some(id)` restricts the query to that owner.
    pub fn event_owner_filter(&self, visibility: EventVisibility) -> Option<Uuid> {
        match (visibility, self.is_admin()) {
            (EventVisibility::Shared, _) | (EventVisibility::Owner, true) => None,
            (EventVisibility::Owner, false) => Some(self.id),
        }
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

        let claims = state.jwt.verify(token)?;
        let id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized)?;

        // Re-check the subject against the live account on every request so
        // that account deletion and role changes take effect immediately,
        // rather than whenever the token happens to expire.
        state.users.get(id).await?.ok_or(AppError::Unauthorized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user(id: Uuid, role: &str) -> AuthUser {
        AuthUser {
            id,
            email: "ranger@example.com".to_string(),
            role: role.to_string(),
        }
    }

    #[test]
    fn shared_visibility_leaves_reads_unscoped() {
        let id = Uuid::new_v4();
        assert_eq!(
            user(id, "operator").event_owner_filter(EventVisibility::Shared),
            None
        );
    }

    #[test]
    fn owner_visibility_scopes_non_admins() {
        let id = Uuid::new_v4();
        assert_eq!(
            user(id, "operator").event_owner_filter(EventVisibility::Owner),
            Some(id)
        );
        assert_eq!(
            user(id, "admin").event_owner_filter(EventVisibility::Owner),
            None
        );
    }
}
