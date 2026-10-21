//! Operator directory endpoints.

use axum::{extract::State, Json};

use crate::{auth::AuthUser, error::AppError, models::user, state::AppState};

/// Maximum number of users returned by the directory endpoint.
const DIRECTORY_LIMIT: i64 = 200;

/// `GET /api/v1/users` — list operators. Admin-only, so every account's email
/// is not exposed to ordinary operators.
pub async fn list(
    State(state): State<AppState>,
    caller: AuthUser,
) -> Result<Json<Vec<user::UserResponse>>, AppError> {
    if !caller.is_admin() {
        return Err(AppError::Forbidden);
    }

    let users = user::list(&state.db, DIRECTORY_LIMIT).await?;
    Ok(Json(users.into_iter().map(Into::into).collect()))
}
