//! Operators (users) and their persistence functions.

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::error::AppError;

/// A user row as stored in the database. Never serialized directly — it
/// contains the password hash.
#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub callsign: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// The public representation of a user returned by the API.
#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub callsign: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            callsign: user.callsign,
            role: user.role,
            created_at: user.created_at,
        }
    }
}

const COLUMNS: &str = "id, email, callsign, password_hash, role, created_at, updated_at";

/// Insert a new user, returning the stored row.
pub async fn create(
    pool: &PgPool,
    email: &str,
    callsign: &str,
    password_hash: &str,
) -> Result<User, AppError> {
    let sql = "insert into users (email, callsign, password_hash) \
               values ($1, $2, $3) \
               returning id, email, callsign, password_hash, role, created_at, updated_at";

    let user = sqlx::query_as::<_, User>(sql)
        .bind(email)
        .bind(callsign)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

/// Look up a user by email (case-insensitive).
pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, AppError> {
    let sql = format!("select {COLUMNS} from users where lower(email) = lower($1)");

    let user = sqlx::query_as::<_, User>(sqlx::AssertSqlSafe(sql))
        .bind(email)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

/// Look up a user by id.
pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, AppError> {
    let sql = format!("select {COLUMNS} from users where id = $1");

    let user = sqlx::query_as::<_, User>(sqlx::AssertSqlSafe(sql))
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

/// List users, most recent first.
pub async fn list(pool: &PgPool, limit: i64) -> Result<Vec<User>, AppError> {
    let sql = format!("select {COLUMNS} from users order by created_at desc limit $1");

    let users = sqlx::query_as::<_, User>(sqlx::AssertSqlSafe(sql))
        .bind(limit)
        .fetch_all(pool)
        .await?;

    Ok(users)
}
