//! Integration tests for SubprocessRunner.

use std::sync::Arc;
use swe_edge_egress_subprocess::{
    SubprocessArgs, SubprocessResult, SubprocessRunner, SubprocessSvc,
};

/// @covers: SubprocessRunner
#[test]
fn test_subprocess_runner_is_object_safe() {
    fn _assert(_: &dyn SubprocessRunner) {}
}

/// @covers: SubprocessRunner
#[test]
fn test_subprocess_runner_can_be_arc_dyn() {
    let _: Arc<dyn SubprocessRunner> = Arc::new(SubprocessSvc::runner());
}

/// @covers: SubprocessRunner::run
#[tokio::test]
async fn test_subprocess_runner_run_denied_on_empty_allow_list() {
    let runner = SubprocessSvc::runner();
    let args = SubprocessArgs::builder().argv(vec!["echo".into()]).build();
    let result = runner.run(args).await;
    assert!(matches!(result, SubprocessResult::Denied { .. }));
}
