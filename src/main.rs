//! ferristak API entrypoint.
//!
//! Commands:
//!   ferristak serve      Start the HTTP server (default).
//!   ferristak migrate    Apply database migrations and exit.

use anyhow::{bail, Context};
use ferristak::{config::Config, db, routes, state::AppState};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load a local .env when present; a no-op in production.
    dotenvy::dotenv().ok();

    init_tracing();

    let config = Config::from_env().context("invalid configuration")?;

    match std::env::args().nth(1).as_deref() {
        None | Some("serve") => serve(config).await,
        Some("migrate") => migrate(config).await,
        Some(other) => bail!("unknown command `{other}` (expected `serve` or `migrate`)"),
    }
}

async fn serve(config: Config) -> anyhow::Result<()> {
    let bind_addr = config
        .socket_addr()
        .context("could not resolve the bind address")?;

    let state = AppState::new(config)
        .await
        .context("failed to initialise application state")?;

    let app = routes::app(state);
    let listener = TcpListener::bind(bind_addr)
        .await
        .with_context(|| format!("failed to bind {bind_addr}"))?;

    tracing::info!(%bind_addr, "ferristak API listening");

    axum::serve(listener, app)
        .await
        .context("HTTP server exited unexpectedly")?;

    Ok(())
}

async fn migrate(config: Config) -> anyhow::Result<()> {
    let pool = db::connect(&config)
        .await
        .context("database connection failed")?;
    db::run_migrations(&pool)
        .await
        .context("migrations failed")?;
    tracing::info!("migrations applied");
    Ok(())
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("ferristak=info,tower_http=info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
