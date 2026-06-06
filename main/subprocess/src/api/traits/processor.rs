//! `Processor` — primary trait for the `processor` service type.

use futures::future::BoxFuture;

use crate::api::vo::subprocess_args::SubprocessArgs;
use crate::api::vo::subprocess_result::SubprocessResult;

/// Primary trait for the `processor` service type.
///
/// Implementors spawn subprocesses and return structured outcomes.
pub trait Processor: Send + Sync + 'static {
    /// Execute a subprocess described by `args`.
    fn process(&self, args: SubprocessArgs) -> BoxFuture<'_, SubprocessResult>;
}
