//! Integration tests for ProcessSvc — the SAF-layer factory.

use std::sync::Arc;
use swe_edge_configbuilder::ConfigBuilder as _;
use swe_edge_egress_subprocess::{ProcessRunner, ProcessSvc, SweEdgeEgressProcess, Validator};

/// @covers: ProcessSvc::config_builder
#[test]
fn test_process_svc_config_builder_has_correct_package_name() {
    assert_eq!(
        ProcessSvc::config_builder().name(),
        "swe-edge-egress-subprocess"
    );
}

/// @covers: ProcessSvc::config_builder
#[test]
fn test_process_svc_config_builder_is_usable() {
    let _loader = ProcessSvc::config_builder().build_loader();
}

/// @covers: ProcessSvc::runner
#[test]
fn test_process_svc_runner_is_object_safe() {
    let _: Arc<dyn ProcessRunner> = Arc::new(ProcessSvc::runner());
}

/// @covers: ProcessSvc::runner
#[tokio::test]
async fn test_process_svc_runner_denies_when_allow_list_empty() {
    use swe_edge_egress_subprocess::{ProcessArgs, ProcessResult};
    let runner = ProcessSvc::runner();
    let args = ProcessArgs::builder().argv(vec!["echo".into()]).build();
    let result = runner.run(args).await;
    assert!(matches!(result, ProcessResult::Denied { .. }));
}

/// @covers: ProcessSvc::service
#[test]
fn test_process_svc_service_is_object_safe() {
    fn _assert(_: &dyn SweEdgeEgressProcess) {}
}

/// @covers: ProcessSvc::validator
#[test]
fn test_process_svc_validator_is_object_safe() {
    fn _assert(_: &dyn Validator) {}
}
