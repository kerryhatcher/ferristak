//! Router-level integration tests that do not require a database.
//!
//! The pool is created lazily, so endpoints that never touch Postgres can be
//! exercised without external services. The user directory is replaced with a
//! fixed map so authentication and authorization run without a DB; requests
//! that fail validation or authorization are rejected before any query runs.

use std::{collections::HashMap, sync::Arc};

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use ferristak::{
    auth::{jwt::JwtService, AuthUser},
    config::{Config, EventVisibility},
    routes,
    state::{AppState, UserDirectory},
};
use http_body_util::BodyExt;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt; // `oneshot`
use uuid::Uuid;

const JWT_SECRET: &str = "test-secret-that-is-long-enough-1234567890";

fn base_config() -> Config {
    Config {
        host: "127.0.0.1".to_string(),
        port: 0,
        database_url: "postgres://user:password@127.0.0.1:1/ferristak".to_string(),
        db_max_connections: 1,
        db_disable_prepared_statements: false,
        jwt_secret: JWT_SECRET.to_string(),
        jwt_ttl_secs: 3_600,
        cors_allowed_origins: Vec::new(),
        run_migrations: false,
        event_visibility: EventVisibility::Shared,
    }
}

/// Build a router whose live user directory contains exactly `users`.
fn test_app(users: Vec<AuthUser>) -> axum::Router {
    let config = base_config();

    // Connects lazily: no network activity until a query runs.
    let db = PgPoolOptions::new()
        .connect_lazy(&config.database_url)
        .expect("valid connection string");

    let jwt = JwtService::new(&config.jwt_secret, config.jwt_ttl_secs);
    let directory: HashMap<Uuid, AuthUser> = users.into_iter().map(|u| (u.id, u)).collect();

    let state = AppState {
        db,
        config: Arc::new(config),
        jwt: Arc::new(jwt),
        users: UserDirectory::Fixed(Arc::new(directory)),
    };

    routes::app(state)
}

/// Issue a token for `id` whose claims say `role` (which may deliberately
/// differ from the role in the live directory).
fn token_for(id: Uuid, role: &str) -> String {
    JwtService::new(JWT_SECRET, 3_600)
        .issue(id, &format!("{role}@example.com"), role)
        .expect("token is issued")
}

fn directory_user(id: Uuid, role: &str) -> AuthUser {
    AuthUser {
        id,
        email: format!("{role}@example.com"),
        role: role.to_string(),
    }
}

#[tokio::test]
async fn health_is_ok() {
    let response = test_app(vec![])
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "ok");
}

#[tokio::test]
async fn unknown_route_is_404() {
    let response = test_app(vec![])
        .oneshot(
            Request::builder()
                .uri("/does-not-exist")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn protected_route_requires_a_token() {
    for uri in ["/api/v1/auth/me", "/api/v1/cot/events"] {
        let response = test_app(vec![])
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "uri: {uri}");
    }
}

#[tokio::test]
async fn cot_json_validation_is_enforced() {
    // Out-of-range latitude and an invalid CoT type: must fail before the DB.
    let payload = serde_json::json!({
        "type": "not a cot type",
        "time": "2026-10-21T15:49:23Z",
        "start": "2026-10-21T15:49:23Z",
        "stale": "2026-10-21T15:54:23Z",
        "how": "m-g",
        "point": { "lat": 200.0, "lon": 0.0 }
    });

    let id = Uuid::new_v4();
    let response = test_app(vec![directory_user(id, "operator")])
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/cot/events")
                .header("content-type", "application/json")
                .header(
                    "authorization",
                    format!("Bearer {}", token_for(id, "operator")),
                )
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn malformed_cot_xml_is_rejected() {
    let id = Uuid::new_v4();
    let response = test_app(vec![directory_user(id, "operator")])
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/cot/events")
                .header("content-type", "application/xml")
                .header(
                    "authorization",
                    format!("Bearer {}", token_for(id, "operator")),
                )
                .body(Body::from("<event>not really a CoT event</event>"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn deleted_user_token_is_rejected() {
    // The token is valid and unexpired, but the account no longer exists.
    let id = Uuid::new_v4();
    let token = token_for(id, "operator");

    let response = test_app(vec![])
        .oneshot(
            Request::builder()
                .uri("/api/v1/auth/me")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn role_comes_from_db_not_token() {
    // The token claims `admin`, but the live account is an `operator`, so the
    // admin-only directory endpoint must reject it.
    let id = Uuid::new_v4();
    let token = token_for(id, "admin");

    let response = test_app(vec![directory_user(id, "operator")])
        .oneshot(
            Request::builder()
                .uri("/api/v1/users")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}
