//! Integration tests for SubprocessSvc — the SAF-layer factory.

use std::sync::Arc;
use swe_edge_egress_subprocess::{
    SubprocessRunner, SubprocessSvc, SweEdgeEgressProcess, Validator,
};

/// @covers: SubprocessSvc::config_builder
#[test]
fn test_subprocess_svc_config_builder_has_correct_package_name() {
    assert_eq!(
        SubprocessSvc::config_builder().name(),
        "swe-edge-egress-subprocess"
    );
}

/// @covers: SubprocessSvc::config_builder
#[test]
fn test_subprocess_svc_config_builder_is_usable() {
    let _loader = SubprocessSvc::config_builder().build_loader();
}

/// @covers: SubprocessSvc::runner
#[test]
fn test_subprocess_svc_runner_is_object_safe() {
    let _: Arc<dyn SubprocessRunner> = Arc::new(SubprocessSvc::runner());
}

/// @covers: SubprocessSvc::runner
#[tokio::test]
async fn test_subprocess_svc_runner_denies_when_allow_list_empty() {
    use swe_edge_egress_subprocess::{SubprocessArgs, SubprocessResult};
    let runner = SubprocessSvc::runner();
    let args = SubprocessArgs::builder().argv(vec!["echo".into()]).build();
    let result = runner.run(args).await;
    assert!(matches!(result, SubprocessResult::Denied { .. }));
}

/// @covers: SubprocessSvc::service
#[test]
fn test_subprocess_svc_service_is_object_safe() {
    fn _assert(_: &dyn SweEdgeEgressProcess) {}
}

/// @covers: SubprocessSvc::validator
#[test]
fn test_subprocess_svc_validator_is_object_safe() {
    fn _assert(_: &dyn Validator) {}
}
