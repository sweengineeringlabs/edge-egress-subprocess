//! Extension hooks for downstream consumers of subprocess execution.
//!
//! Implement [`SubprocessRunnerExtension`] to provide custom subprocess execution
//! semantics (e.g. sandbox integration, remote execution, audit logging).
//! Wire your implementation into the stack via [`SubprocessSvc::with_runner`].
//!
//! [`SubprocessSvc::with_runner`]: crate::SubprocessSvc::with_runner

pub use crate::api::traits::subprocess::subprocess_runner_extension::SubprocessRunnerExtension;
