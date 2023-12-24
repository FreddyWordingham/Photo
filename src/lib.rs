//! Library core.

// Lints.
#![warn(
    clippy::all,
    clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::restriction,
    clippy::style,
    clippy::suspicious
)]

// Constants
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
