//! HS256 JSON Web Token issuance and verification.

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

/// Claims embedded in every API token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject: the user id.
    pub sub: String,
    /// User email (convenience for clients).
    pub email: String,
    /// User role (`operator` or `admin`).
    pub role: String,
    /// Issued-at (Unix seconds).
    pub iat: i64,
    /// Expiry (Unix seconds).
    pub exp: i64,
}

/// Signs and verifies API tokens with a symmetric secret.
#[derive(Clone)]
pub struct JwtService {
    encoding: EncodingKey,
    decoding: DecodingKey,
    validation: Validation,
    ttl_secs: i64,
}

impl JwtService {
    /// Build a service from a shared secret and token lifetime.
    pub fn new(secret: &str, ttl_secs: i64) -> Self {
        let secret = secret.as_bytes();
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
            // `Validation::new` requires `exp` and enforces the HS256 algorithm.
            validation: Validation::new(Algorithm::HS256),
            ttl_secs,
        }
    }

    /// Issue a token for the given user.
    pub fn issue(&self, user_id: Uuid, email: &str, role: &str) -> Result<String, AppError> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            iat: now.timestamp(),
            exp: (now + Duration::seconds(self.ttl_secs)).timestamp(),
        };

        encode(&Header::default(), &claims, &self.encoding)
            .map_err(|err| AppError::Internal(anyhow::Error::new(err)))
    }

    /// Verify a token and return its claims.
    ///
    /// Any failure (bad signature, expired, malformed) is reported as
    /// `Unauthorized` so that clients cannot distinguish the cause.
    pub fn verify(&self, token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(token, &self.decoding, &self.validation)
            .map(|data| data.claims)
            .map_err(|_| AppError::Unauthorized)
    }
}
