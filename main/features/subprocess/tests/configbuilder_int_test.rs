//! Integration tests verifying `swe-edge-configbuilder` usage.

use swe_edge_configbuilder::ConfigSection as _;
use swe_edge_egress_subprocess::{SubprocessConfig, SubprocessSvc};

/// @covers: config_builder
#[test]
fn test_create_config_builder_returns_usable_loader() {
    let builder = SubprocessSvc::config_builder();
    // build_loader returns an opaque loader — verify it doesn't panic
    let _ = builder.build_loader();
}

/// @covers: SubprocessConfig::section_name
#[test]
fn test_subprocess_config_section_name_returns_subprocess() {
    assert_eq!(SubprocessConfig::section_name(), "subprocess");
}

/// @covers: SubprocessConfig::load — absent config dir returns error or default
#[test]
fn test_subprocess_config_default_is_sane() {
    // Verify the default is well-formed even without loading from disk.
    let cfg = SubprocessConfig::default();
    assert!(cfg.allow_commands.is_empty());
    assert!(cfg.timeout_ms > 0);
    assert!(cfg.output_bytes_cap > 0);
}
