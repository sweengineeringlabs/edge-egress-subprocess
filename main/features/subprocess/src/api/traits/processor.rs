//! `Processor` — primary trait for the `processor` service type.

use futures::future::BoxFuture;

use crate::api::types::process::process_args::ProcessArgs;
use crate::api::types::process::process_result::ProcessResult;

/// Primary trait for the `processor` service type.
///
/// Implementors spawn subprocesses and return structured outcomes.
pub trait Processor: Send + Sync + 'static {
    /// Execute a subprocess described by `args`.
    fn process(&self, args: ProcessArgs) -> BoxFuture<'_, ProcessResult>;
}
