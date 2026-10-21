//! ferristak — a Rust-based Team Awareness Kit API.
//!
//! The crate is split into a library (this file) and a binary (`src/main.rs`).
//! Keeping the logic in a library makes it straightforward to unit/integration
//! test handlers without booting the full server.

pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod models;
pub mod routes;
pub mod state;
