//! Integration tests for application_config_builder.

use swe_edge_configbuilder::ConfigBuilder as _;
use swe_edge_egress_subprocess::ProcessSvc;

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_has_correct_package_name() {
    let builder = ProcessSvc::config_builder();
    assert_eq!(builder.name(), "swe-edge-egress-subprocess");
}
