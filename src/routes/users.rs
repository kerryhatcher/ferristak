//! Operator directory endpoints.

use axum::{extract::State, Json};

use crate::{auth::AuthUser, error::AppError, models::user, state::AppState};

/// Maximum number of users returned by the directory endpoint.
const DIRECTORY_LIMIT: i64 = 200;

/// `GET /api/v1/users` — list operators (requires authentication).
pub async fn list(
    State(state): State<AppState>,
    _caller: AuthUser,
) -> Result<Json<Vec<user::UserResponse>>, AppError> {
    let users = user::list(&state.db, DIRECTORY_LIMIT).await?;
    Ok(Json(users.into_iter().map(Into::into).collect()))
}
