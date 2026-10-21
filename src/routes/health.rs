//! Liveness and readiness probes, plus the JSON 404 fallback.

use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{error::AppError, state::AppState};

/// Liveness: the process is up. Does not touch dependencies.
pub async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": env!("CARGO_PKG_NAME"),
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// Readiness: dependencies (Postgres) are reachable.
pub async fn ready(State(state): State<AppState>) -> Result<Json<Value>, AppError> {
    ping(&state.db).await?;
    Ok(Json(json!({ "status": "ready" })))
}

/// Fallback handler for unknown routes.
pub async fn not_found() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "error": { "code": "not_found", "message": "route not found" }
        })),
    )
}

async fn ping(pool: &PgPool) -> Result<(), AppError> {
    sqlx::query("select 1").execute(pool).await?;
    Ok(())
}
