//! Argon2id password hashing.
//!
//! Hashes are stored in PHC string format, which embeds the algorithm,
//! parameters and a random salt, e.g.
//! `$argon2id$v=19$m=19456,t=2,p=1$<salt>$<hash>`.

use argon2::{Argon2, PasswordHasher, PasswordVerifier};

use crate::error::AppError;

/// Number of random salt bytes. 16 bytes / 128 bits is the recommended minimum.
const SALT_LEN: usize = 16;

/// Hash a plaintext password, generating a fresh random salt.
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt: [u8; SALT_LEN] = rand::random();

    Argon2::default()
        .hash_password_with_salt(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|err| AppError::Internal(anyhow::anyhow!("password hashing failed: {err}")))
}

/// Verify a plaintext password against a stored PHC hash.
///
/// Returns `false` for malformed stored hashes rather than erroring, so that a
/// corrupted record cannot be distinguished from a wrong password by clients.
pub fn verify_password(password: &str, stored_hash: &str) -> bool {
    Argon2::default()
        .verify_password(password.as_bytes(), stored_hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips() {
        let hash = hash_password("correct horse battery staple").unwrap();
        assert!(hash.starts_with("$argon2"));
        assert!(verify_password("correct horse battery staple", &hash));
        assert!(!verify_password("wrong password", &hash));
    }

    #[test]
    fn salts_are_unique() {
        let a = hash_password("same password").unwrap();
        let b = hash_password("same password").unwrap();
        assert_ne!(a, b, "each hash must use a fresh salt");
    }

    #[test]
    fn malformed_hash_is_rejected() {
        assert!(!verify_password("password", "not-a-valid-phc-string"));
    }
}
