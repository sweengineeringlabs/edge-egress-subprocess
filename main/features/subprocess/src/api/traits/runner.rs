//! `ProcessRunner` — public trait for subprocess execution.

use futures::future::BoxFuture;

use crate::api::types::process::args::ProcessArgs;
use crate::api::types::process::outcome::ProcessResult;

/// Outbound subprocess execution — the process analogue of `GrpcEgressClient`.
///
/// Implementors spawn an external binary, enforce the supplied policy, and
/// return a structured [`ProcessResult`].  The trait is object-safe and
/// `Send + Sync` so it can be stored behind `Arc<dyn ProcessRunner>`.
///
/// Consumers call [`crate::process_runner`] and receive `impl ProcessRunner`.
/// The concrete type stays `pub(crate)` in `core/`; callers never name it.
pub trait ProcessRunner: Send + Sync + 'static {
    /// Spawn the subprocess described by `args` and return the outcome.
    ///
    /// Never returns `Err` — all failure modes are encoded in [`ProcessResult`]
    /// variants so callers can match exhaustively.
    fn run(&self, args: ProcessArgs) -> BoxFuture<'_, ProcessResult>;
}
