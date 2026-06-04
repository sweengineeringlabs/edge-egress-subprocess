//! Integration tests for SubprocessConfigBuilder.

use swe_edge_egress_subprocess::SubprocessConfig;

/// @covers: SubprocessConfigBuilder::build
#[test]
fn test_subprocess_config_builder_default_has_empty_allow_commands() {
    let cfg = SubprocessConfig::default();
    assert!(cfg.allow_commands.is_empty());
}
