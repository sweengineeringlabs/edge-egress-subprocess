//! `SubprocessRunnerExtension` — downstream extension point for subprocess execution.

use futures::future::BoxFuture;

use crate::api::vo::subprocess_args::SubprocessArgs;
use crate::api::vo::subprocess_result::SubprocessResult;

/// Extension point for downstream implementors of subprocess execution.
///
/// Consumers that need custom execution semantics (e.g. sandbox integration,
/// remote execution) implement this trait and wire it up via a custom factory.
pub trait SubprocessRunnerExtension: Send + Sync + 'static {
    /// Execute a subprocess with custom logic.
    fn run_extended(&self, args: SubprocessArgs) -> BoxFuture<'_, SubprocessResult>;
}
