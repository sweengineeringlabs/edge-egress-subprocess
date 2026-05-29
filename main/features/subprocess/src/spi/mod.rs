//! Extension hooks for downstream consumers.
//!
//! Implement [`ProcessRunnerExtension`] to plug in a custom subprocess
//! execution strategy as an alternative to the default runner.

pub use crate::api::traits::runner_extension::ProcessRunnerExtension;
