//! Router-level integration tests that do not require a database.
//!
//! The pool is created lazily, so endpoints that never touch Postgres can be
//! exercised without external services. Requests that fail validation are
//! rejected before any query runs.

use std::sync::Arc;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use ferristak::{auth::jwt::JwtService, config::Config, routes, state::AppState};
use http_body_util::BodyExt;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt; // `oneshot`
use uuid::Uuid;

const JWT_SECRET: &str = "test-secret-that-is-long-enough-1234567890";

fn test_app() -> axum::Router {
    let config = Config {
        host: "127.0.0.1".to_string(),
        port: 0,
        database_url: "postgres://user:password@127.0.0.1:1/ferristak".to_string(),
        db_max_connections: 1,
        db_disable_prepared_statements: false,
        jwt_secret: JWT_SECRET.to_string(),
        jwt_ttl_secs: 3_600,
        cors_allowed_origins: Vec::new(),
        run_migrations: false,
    };

    // Connects lazily: no network activity until a query runs.
    let db = PgPoolOptions::new()
        .connect_lazy(&config.database_url)
        .expect("valid connection string");

    let jwt = JwtService::new(&config.jwt_secret, config.jwt_ttl_secs);
    let state = AppState {
        db,
        config: Arc::new(config),
        jwt: Arc::new(jwt),
    };

    routes::app(state)
}

fn token() -> String {
    JwtService::new(JWT_SECRET, 3_600)
        .issue(Uuid::new_v4(), "ranger@example.com", "operator")
        .expect("token is issued")
}

#[tokio::test]
async fn health_is_ok() {
    let response = test_app()
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
    let response = test_app()
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
        let response = test_app()
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

    let response = test_app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/cot/events")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token()))
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn malformed_cot_xml_is_rejected() {
    let response = test_app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/cot/events")
                .header("content-type", "application/xml")
                .header("authorization", format!("Bearer {}", token()))
                .body(Body::from("<event>not really a CoT event</event>"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
