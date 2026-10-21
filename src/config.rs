//! Runtime configuration, loaded from the environment.
//!
//! Secrets are never hard-coded: in production Fly.io injects them as secrets,
//! while local development uses a gitignored `.env` file.

use std::{
    env,
    net::{IpAddr, SocketAddr},
};

/// Configuration error raised while reading the environment.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct ConfigError(String);

impl ConfigError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

/// Who may read a stored CoT event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventVisibility {
    /// Every authenticated caller sees every event. This is the correct mode
    /// for a single team sharing one situational picture (the TAK use case).
    Shared,
    /// Callers see only events they own. Admins still see everything.
    Owner,
}

fn parse_event_visibility(value: &str) -> Result<EventVisibility, ConfigError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "shared" => Ok(EventVisibility::Shared),
        "owner" => Ok(EventVisibility::Owner),
        other => Err(ConfigError::new(format!(
            "EVENT_VISIBILITY must be \"shared\" or \"owner\", got `{other}`"
        ))),
    }
}

/// Application configuration.
#[derive(Debug, Clone)]
pub struct Config {
    /// Interface to listen on.
    pub host: String,
    /// TCP port to listen on.
    pub port: u16,
    /// Postgres connection string (Supabase pooler or direct).
    pub database_url: String,
    /// Maximum number of pooled connections.
    pub db_max_connections: u32,
    /// Disable server-side prepared statements (required for Supavisor
    /// transaction pooling on port 6543).
    pub db_disable_prepared_statements: bool,
    /// HMAC secret used to sign and verify API JWTs.
    pub jwt_secret: String,
    /// Lifetime of issued JWTs, in seconds.
    pub jwt_ttl_secs: i64,
    /// Allowed browser origins. Empty means permissive (development only).
    pub cors_allowed_origins: Vec<String>,
    /// Run embedded migrations on startup.
    pub run_migrations: bool,
    /// Who may read stored CoT events.
    pub event_visibility: EventVisibility,
}

impl Config {
    /// Read configuration from the process environment.
    pub fn from_env() -> Result<Self, ConfigError> {
        let database_url = required("DATABASE_URL")?;

        let jwt_secret = required("JWT_SECRET")?;
        if jwt_secret.len() < 32 {
            return Err(ConfigError::new(
                "JWT_SECRET must be at least 32 bytes long",
            ));
        }

        // `BIND_ADDR` takes precedence when set (useful for Fly, which also
        // injects `PORT`).
        let (host, port) = match env::var("BIND_ADDR") {
            Ok(bind_addr) => {
                let parsed: SocketAddr = bind_addr.parse().map_err(|_| {
                    ConfigError::new(format!(
                        "BIND_ADDR is not a valid socket address: {bind_addr}"
                    ))
                })?;
                (parsed.ip().to_string(), parsed.port())
            }
            Err(_) => (
                env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                parse_or("PORT", 8080)?,
            ),
        };

        // Secure by default: callers only see their own events unless the
        // deployment explicitly opts into a shared team picture.
        let event_visibility = match env::var("EVENT_VISIBILITY") {
            Ok(value) => parse_event_visibility(&value)?,
            Err(_) => EventVisibility::Owner,
        };

        let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_default()
            .split(',')
            .map(str::trim)
            .filter(|origin| !origin.is_empty())
            .map(ToOwned::to_owned)
            .collect();

        Ok(Self {
            host,
            port,
            database_url,
            db_max_connections: parse_or("DB_MAX_CONNECTIONS", 5)?,
            db_disable_prepared_statements: parse_or("DB_DISABLE_PREPARED_STATEMENTS", false)?,
            jwt_secret,
            jwt_ttl_secs: parse_or("JWT_TTL_SECS", 86_400)?,
            cors_allowed_origins,
            run_migrations: parse_or("RUN_MIGRATIONS", true)?,
            event_visibility,
        })
    }

    /// Resolve the configured host/port into a socket address.
    pub fn socket_addr(&self) -> Result<SocketAddr, ConfigError> {
        let ip: IpAddr = self.host.parse().map_err(|_| {
            ConfigError::new(format!("HOST is not a valid IP address: {}", self.host))
        })?;
        Ok(SocketAddr::new(ip, self.port))
    }
}

fn required(key: &str) -> Result<String, ConfigError> {
    env::var(key).map_err(|_| ConfigError::new(format!("{key} must be set")))
}

fn parse_or<T>(key: &str, default: T) -> Result<T, ConfigError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    match env::var(key) {
        Ok(value) => value
            .parse()
            .map_err(|err| ConfigError::new(format!("{key} has an invalid value: {err}"))),
        Err(_) => Ok(default),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_event_visibility() {
        assert_eq!(
            parse_event_visibility("shared").unwrap(),
            EventVisibility::Shared
        );
        assert_eq!(
            parse_event_visibility(" OWNER ").unwrap(),
            EventVisibility::Owner
        );
        assert!(parse_event_visibility("everyone").is_err());
    }
}
