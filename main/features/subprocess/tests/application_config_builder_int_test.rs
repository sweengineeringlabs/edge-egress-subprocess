//! Integration tests for application_config_builder.

use swe_edge_egress_subprocess::SubprocessSvc;

/// @covers: config_builder
#[test]
fn test_create_config_builder_has_correct_package_name() {
    let builder = SubprocessSvc::config_builder();
    assert_eq!(builder.name(), "swe-edge-egress-subprocess");
}
