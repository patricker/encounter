//! Crate error types.

use thiserror::Error;

/// Errors that can occur in the `encounter` crate.
#[derive(Debug, Error)]
pub enum Error {
    /// A catalog file could not be parsed.
    #[error("catalog parse error in {path}: {reason}")]
    CatalogParse {
        /// Path of the file that failed.
        path: String,
        /// What went wrong.
        reason: String,
    },

    /// A resolution protocol was invoked with invalid input.
    #[error("resolution error: {0}")]
    Resolution(String),

    /// An IO error during catalog loading.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// A TOML deserialization error.
    #[error("toml error: {0}")]
    Toml(#[from] toml::de::Error),
}
