//! `ProcessRunnerExtension` — downstream extension point for subprocess execution.

use futures::future::BoxFuture;

use crate::api::types::process::process_args::ProcessArgs;
use crate::api::types::process::process_result::ProcessResult;

/// Extension point for downstream implementors of subprocess execution.
///
/// Consumers that need custom execution semantics (e.g. sandbox integration,
/// remote execution) implement this trait and wire it up via a custom factory.
pub trait ProcessRunnerExtension: Send + Sync + 'static {
    /// Execute a subprocess with custom logic.
    fn run_extended(&self, args: ProcessArgs) -> BoxFuture<'_, ProcessResult>;
}
