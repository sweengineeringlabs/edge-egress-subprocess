//! Extension hooks for downstream consumers of subprocess execution.
//!
//! Implement [`ProcessRunnerExtension`] to provide custom subprocess execution
//! semantics (e.g. sandbox integration, remote execution, audit logging).
//! Wire your implementation into the stack via [`ProcessSvc::with_runner`].
//!
//! [`ProcessSvc::with_runner`]: crate::ProcessSvc::with_runner

pub use crate::api::traits::runner_extension::ProcessRunnerExtension;
