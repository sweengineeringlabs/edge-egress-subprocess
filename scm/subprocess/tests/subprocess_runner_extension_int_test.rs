//! Integration tests for SubprocessRunnerExtension.

use futures::future::BoxFuture;
use swe_edge_egress_subprocess::{
    SubprocessArgs, SubprocessResult, SubprocessRunner, SubprocessRunnerExtension, SubprocessSvc,
};

#[derive(Debug)]
struct AlwaysDenyExt;

impl SubprocessRunnerExtension for AlwaysDenyExt {
    fn run_extended(&self, args: SubprocessArgs) -> BoxFuture<'_, SubprocessResult> {
        let cmd = args.argv.first().cloned().unwrap_or_default();
        Box::pin(async move { SubprocessResult::Denied { command: cmd } })
    }
}

/// @covers: SubprocessRunnerExtension
#[test]
fn test_subprocess_runner_extension_is_object_safe() {
    fn _assert(_: &dyn SubprocessRunnerExtension) {}
}

/// @covers: SubprocessSvc::with_runner
#[tokio::test]
async fn test_subprocess_runner_extension_wired_via_with_runner() {
    let runner = SubprocessSvc::with_runner(AlwaysDenyExt);
    let args = SubprocessArgs::builder()
        .argv(vec!["echo".into()])
        .allow_commands(vec!["echo".into()])
        .build();
    let result = runner.run(args).await;
    assert!(matches!(result, SubprocessResult::Denied { .. }));
}
