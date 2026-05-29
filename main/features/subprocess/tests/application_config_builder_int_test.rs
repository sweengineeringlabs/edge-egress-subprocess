//! Integration tests for application_config_builder.

use swe_edge_configbuilder::{ConfigBuilder as _, ConfigLoaderFactory};

/// @covers: ApplicationConfigBuilder
#[test]
fn test_application_config_builder_creates_usable_builder() {
    let builder = ConfigLoaderFactory::create_config_builder()
        .with_name("swe-edge-egress-subprocess")
        .with_version("0.1.0");
    assert_eq!(builder.name(), "swe-edge-egress-subprocess");
}
