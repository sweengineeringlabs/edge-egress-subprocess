//! Integration tests for SubprocessRunnerExtension.

use std::sync::Arc;
use swe_edge_egress_subprocess::SubprocessRunnerExtension;

/// @covers: SubprocessRunnerExtension
#[test]
fn test_subprocess_runner_extension_is_object_safe() {
    fn _assert(_: &dyn SubprocessRunnerExtension) {}
}

/// @covers: SubprocessRunnerExtension
#[test]
fn test_subprocess_runner_extension_can_be_stored_as_arc_dyn() {
    use futures::future::BoxFuture;
    use swe_edge_egress_subprocess::{SubprocessArgs, SubprocessResult};

    #[derive(Debug)]
    struct NoopExt;
    impl SubprocessRunnerExtension for NoopExt {
        fn run_extended(&self, args: SubprocessArgs) -> BoxFuture<'_, SubprocessResult> {
            Box::pin(async move {
                SubprocessResult::Denied {
                    command: args.argv.first().cloned().unwrap_or_default(),
                }
            })
        }
    }

    let _: Arc<dyn SubprocessRunnerExtension> = Arc::new(NoopExt);
}
