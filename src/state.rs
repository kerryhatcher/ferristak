//! Shared application state.

use std::sync::Arc;

use sqlx::PgPool;

use crate::{auth::jwt::JwtService, config::Config, db};

/// Shared state injected into every handler via axum's `State` extractor.
///
/// Cloning is cheap: the pool is internally reference-counted and the rest is
/// behind `Arc`.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
    pub jwt: Arc<JwtService>,
}

impl AppState {
    /// Connect to the database, optionally run migrations, and build the JWT
    /// service.
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let db = db::connect(&config).await?;

        if config.run_migrations {
            tracing::info!("running database migrations");
            db::run_migrations(&db).await?;
        }

        let jwt = JwtService::new(&config.jwt_secret, config.jwt_ttl_secs);

        Ok(Self {
            db,
            config: Arc::new(config),
            jwt: Arc::new(jwt),
        })
    }
}
