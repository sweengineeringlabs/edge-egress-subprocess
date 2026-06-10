//! `SubprocessRunner` — public trait for subprocess execution.

use futures::future::BoxFuture;

use crate::api::types::subprocess::args::SubprocessArgs;
use crate::api::types::subprocess::result::SubprocessResult;

/// Outbound subprocess execution — the subprocess analogue of `GrpcEgressClient`.
///
/// Implementors spawn an external binary, enforce the supplied policy, and
/// return a structured [`SubprocessResult`].  The trait is object-safe and
/// `Send + Sync` so it can be stored behind `Arc<dyn SubprocessRunner>`.
///
/// Consumers call [`crate::subprocess_runner`] and receive `impl SubprocessRunner`.
/// The concrete type stays `pub(crate)` in `core/`; callers never name it.
pub trait SubprocessRunner: Send + Sync + 'static {
    /// Spawn the subprocess described by `args` and return the outcome.
    ///
    /// Never returns `Err` — all failure modes are encoded in [`SubprocessResult`]
    /// variants so callers can match exhaustively.
    fn run(&self, args: SubprocessArgs) -> BoxFuture<'_, SubprocessResult>;
}
