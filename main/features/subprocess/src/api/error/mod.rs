//! Error types for subprocess execution.

pub mod error;
pub mod isolation_error;

pub use error::Error;
pub use isolation_error::IsolationError;
