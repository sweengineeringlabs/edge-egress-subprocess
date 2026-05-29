//! `IsolationProfile` — OS-level isolation contract for subprocess execution.

use crate::api::error::isolation_error::IsolationError;

/// OS-level isolation applied around a subprocess invocation.
///
/// Implementations are `Send + Sync` so they can be stored in `ProcessArgs`
/// and shared across concurrent [`ProcessRunner`] calls.
///
/// [`ProcessRunner`]: crate::ProcessRunner
pub trait IsolationProfile: std::fmt::Debug + Send + Sync + 'static {
    /// Stable identifier for this profile, used in audit logs and error messages.
    fn name(&self) -> &str;

    /// Configure `cmd` before the process is spawned.
    ///
    /// Called on the parent process before `fork`/`exec`. Use this for
    /// mechanisms that must be established before the child starts executing
    /// (e.g. `seccomp-bpf` via `pre_exec` on Linux).
    ///
    /// The default implementation is a no-op.
    fn configure(&self, _cmd: &mut tokio::process::Command) -> Result<(), IsolationError> {
        Ok(())
    }

    /// Apply isolation to the spawned child process.
    ///
    /// Called immediately after spawn, before the runner waits on the child.
    /// Use this for mechanisms that operate on a live process handle (e.g.
    /// Windows Job Objects).
    ///
    /// If this returns `Err`, the child is killed and
    /// [`ProcessResult::IsolationFailed`] is returned to the caller.
    ///
    /// The default implementation is a no-op.
    ///
    /// [`ProcessResult::IsolationFailed`]: crate::ProcessResult::IsolationFailed
    fn apply(&self, _child: &mut tokio::process::Child) -> Result<(), IsolationError> {
        Ok(())
    }
}
