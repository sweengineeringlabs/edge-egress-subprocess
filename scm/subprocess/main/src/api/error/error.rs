//! `Error` — crate-level error type.

/// Errors that can occur in swe_edge_egress_subprocess.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// An operation failed.
    #[error("Operation failed: {message}")]
    Operation {
        /// Description of what failed.
        message: String,
    },
}
