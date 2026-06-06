//! Error types for subprocess execution.

#[allow(clippy::module_inception)]
pub mod error;
pub mod isolation_error;

pub use error::Error;
pub use isolation_error::IsolationError;
