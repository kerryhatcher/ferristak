//! Postgres connectivity and migrations.

use std::{path::Path, time::Duration};

use sqlx::{
    migrate::Migrator,
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

use crate::config::Config;

/// Build a connection pool for the configured database.
pub async fn connect(config: &Config) -> anyhow::Result<PgPool> {
    let mut options: PgConnectOptions = config
        .database_url
        .parse()
        .map_err(|err| anyhow::anyhow!("invalid DATABASE_URL: {err}"))?;

    if config.db_disable_prepared_statements {
        options = options.statement_cache_capacity(0);
    }

    let pool = PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .acquire_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(30 * 60))
        .idle_timeout(Duration::from_secs(10 * 60))
        .connect_with(options)
        .await?;

    Ok(pool)
}

/// Apply all migrations found in `./migrations`.
///
/// Migrations are tracked in the `_sqlx_migrations` table, so this is
/// idempotent and safe to run on every boot.
pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    migrator.run(pool).await?;
    Ok(())
}
