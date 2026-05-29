//! Integration tests for ProcessRunnerExtension.

use std::sync::Arc;
use swe_edge_egress_subprocess::ProcessRunnerExtension;

/// @covers: ProcessRunnerExtension
#[test]
fn test_process_runner_extension_is_object_safe() {
    fn _assert(_: &dyn ProcessRunnerExtension) {}
}

/// @covers: ProcessRunnerExtension
#[test]
fn test_process_runner_extension_can_be_stored_as_arc_dyn() {
    use futures::future::BoxFuture;
    use swe_edge_egress_subprocess::{ProcessArgs, ProcessResult};

    #[derive(Debug)]
    struct NoopExt;
    impl ProcessRunnerExtension for NoopExt {
        fn run_extended(&self, args: ProcessArgs) -> BoxFuture<'_, ProcessResult> {
            Box::pin(async move {
                ProcessResult::Denied {
                    command: args.argv.first().cloned().unwrap_or_default(),
                }
            })
        }
    }

    let _: Arc<dyn ProcessRunnerExtension> = Arc::new(NoopExt);
}
