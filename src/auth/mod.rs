//! Authentication: password hashing, JWT issuance/verification, and the
//! `AuthUser` request extractor.

pub mod extractor;
pub mod jwt;
pub mod password;

pub use extractor::AuthUser;
pub use jwt::{Claims, JwtService};
