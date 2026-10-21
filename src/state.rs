//! Shared application state.

use std::{collections::HashMap, sync::Arc};

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::jwt::JwtService, auth::AuthUser, config::Config, db, error::AppError, models::user,
};

/// Resolves an authenticated subject to its *current* account record. The
/// database-backed directory is used in production; tests inject a fixed map so
/// they can exercise the pipeline without a live Postgres.
#[derive(Clone)]
pub enum UserDirectory {
    Database(PgPool),
    Fixed(Arc<HashMap<Uuid, AuthUser>>),
}

impl UserDirectory {
    /// Look up the caller by id. `None` means the account no longer exists.
    pub async fn get(&self, id: Uuid) -> Result<Option<AuthUser>, AppError> {
        match self {
            Self::Database(pool) => {
                let row = user::find_by_id(pool, id).await?;
                Ok(row.map(|user| AuthUser {
                    id: user.id,
                    email: user.email,
                    role: user.role,
                }))
            }
            Self::Fixed(users) => Ok(users.get(&id).cloned()),
        }
    }
}

/// Shared state injected into every handler via axum's `State` extractor.
///
/// Cloning is cheap: the pool is internally reference-counted and the rest is
/// behind `Arc`.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
    pub jwt: Arc<JwtService>,
    /// Revalidates the token subject against the live account on every request.
    pub users: UserDirectory,
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
            users: UserDirectory::Database(db.clone()),
            db,
            config: Arc::new(config),
            jwt: Arc::new(jwt),
        })
    }
}
