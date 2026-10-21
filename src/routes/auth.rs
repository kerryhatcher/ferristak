//! Registration, login, and current-user endpoints.

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    auth::{password, AuthUser},
    error::AppError,
    models::user,
    state::AppState,
};

/// Payload for `POST /api/v1/auth/register`.
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "email is not valid"))]
    pub email: String,
    #[validate(length(min = 2, max = 64, message = "callsign must be 2-64 characters"))]
    pub callsign: String,
    #[validate(length(min = 8, max = 128, message = "password must be 8-128 characters"))]
    pub password: String,
}

/// Payload for `POST /api/v1/auth/login`.
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "email is not valid"))]
    pub email: String,
    #[validate(length(min = 1, message = "password is required"))]
    pub password: String,
}

/// A successful authentication response.
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: &'static str,
    pub user: user::UserResponse,
}

/// `POST /api/v1/auth/register` — create an operator account and sign in.
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    payload.validate().map_err(validation_error)?;

    // Store emails canonically so that lookups are case-insensitive and a
    // single person cannot register twice with different casing.
    let email = payload.email.trim().to_lowercase();
    let password_hash = password::hash_password(&payload.password)?;
    let created = user::create(&state.db, &email, &payload.callsign, &password_hash).await?;

    let token = state.jwt.issue(created.id, &created.email, &created.role)?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            token_type: "Bearer",
            user: created.into(),
        }),
    ))
}

/// `POST /api/v1/auth/login` — exchange credentials for a JWT.
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    payload.validate().map_err(validation_error)?;

    let found = user::find_by_email(&state.db, &payload.email).await?;

    // Always run Argon2, even for an unknown email, so login timing does not
    // reveal which accounts exist.
    let stored = found.as_ref().map(|user| user.password_hash.as_str());
    if !password::verify_password_or_dummy(&payload.password, stored) {
        return Err(AppError::Unauthorized);
    }

    let found = found.ok_or(AppError::Unauthorized)?;
    let token = state.jwt.issue(found.id, &found.email, &found.role)?;

    Ok(Json(AuthResponse {
        token,
        token_type: "Bearer",
        user: found.into(),
    }))
}

/// `GET /api/v1/auth/me` — return the authenticated caller.
pub async fn me(
    State(state): State<AppState>,
    caller: AuthUser,
) -> Result<Json<user::UserResponse>, AppError> {
    let found = user::find_by_id(&state.db, caller.id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(found.into()))
}

fn validation_error(errors: validator::ValidationErrors) -> AppError {
    // Collapse field errors into a readable, single-line message.
    let message = errors
        .field_errors()
        .iter()
        .flat_map(|(field, errors)| {
            errors.iter().map(move |error| {
                error
                    .message
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| format!("{field} is invalid"))
            })
        })
        .collect::<Vec<_>>()
        .join("; ");

    if message.is_empty() {
        AppError::validation("request is invalid")
    } else {
        AppError::validation(message)
    }
}
