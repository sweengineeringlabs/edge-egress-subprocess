//! Integration tests for SubprocessResult.

use swe_edge_egress_subprocess::{
    SubprocessArgs, SubprocessResult, SubprocessRunner, SubprocessSvc,
};

/// @covers: SubprocessResult::Denied
#[tokio::test]
async fn test_subprocess_result_denied_on_blocked_command() {
    let runner = SubprocessSvc::runner();
    let args = SubprocessArgs::builder().argv(vec!["echo".into()]).build();
    assert!(matches!(
        runner.run(args).await,
        SubprocessResult::Denied { .. }
    ));
}

/// @covers: SubprocessResult::SpawnFailed
#[tokio::test]
async fn test_subprocess_result_spawn_failed_for_nonexistent_binary() {
    let runner = SubprocessSvc::runner();
    let args = SubprocessArgs::builder()
        .argv(vec!["__swe_nonexistent__".into()])
        .allow_commands(vec!["__swe_nonexistent__".into()])
        .build();
    assert!(matches!(
        runner.run(args).await,
        SubprocessResult::SpawnFailed { .. }
    ));
}
