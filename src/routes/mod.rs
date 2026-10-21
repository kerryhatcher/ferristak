//! HTTP routing. Builds the axum `Router` and assembles all endpoints.

pub mod auth;
pub mod cot;
pub mod health;
pub mod users;

use axum::{
    extract::DefaultBodyLimit,
    http::{header, HeaderValue, Method},
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{AllowOrigin, Any, CorsLayer},
    trace::TraceLayer,
};

use crate::state::AppState;

/// Maximum accepted request body (CoT events are small).
const MAX_BODY_BYTES: usize = 256 * 1024;

/// Assemble the application router.
pub fn app(state: AppState) -> Router {
    let cors = build_cors(&state.config.cors_allowed_origins);

    Router::new()
        .route("/health", get(health::health))
        .route("/ready", get(health::ready))
        .route("/api/v1/auth/register", post(auth::register))
        .route("/api/v1/auth/login", post(auth::login))
        .route("/api/v1/auth/me", get(auth::me))
        .route("/api/v1/users", get(users::list))
        .route("/api/v1/cot/events", get(cot::list).post(cot::create))
        .route("/api/v1/cot/events/latest", get(cot::latest))
        .route("/api/v1/cot/events/{id}", get(cot::get))
        .fallback(health::not_found)
        .layer(DefaultBodyLimit::max(MAX_BODY_BYTES))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}

fn build_cors(allowed_origins: &[String]) -> CorsLayer {
    let base = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

    if allowed_origins.is_empty() {
        // No explicit allow-list configured: permissive is convenient for
        // local development. Production should always set the allow-list.
        return base.allow_origin(Any);
    }

    let origins: Vec<HeaderValue> = allowed_origins
        .iter()
        .filter_map(|origin| match origin.parse() {
            Ok(value) => Some(value),
            Err(_) => {
                tracing::warn!(%origin, "ignoring invalid CORS origin");
                None
            }
        })
        .collect();

    base.allow_origin(AllowOrigin::list(origins))
}
