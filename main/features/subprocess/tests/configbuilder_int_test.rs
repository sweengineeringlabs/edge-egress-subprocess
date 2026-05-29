//! Integration tests verifying `swe-edge-configbuilder` usage.

use swe_edge_configbuilder::ConfigSection as _;
use swe_edge_egress_subprocess::{ProcessConfig, ProcessSvc};

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_returns_usable_loader() {
    use swe_edge_configbuilder::ConfigBuilder as _;
    let builder = ProcessSvc::config_builder();
    // build_loader returns an opaque loader — verify it doesn't panic
    let _ = builder.build_loader();
}

/// @covers: ProcessConfig::section_name
#[test]
fn test_process_config_section_name_returns_process() {
    assert_eq!(ProcessConfig::section_name(), "process");
}

/// @covers: ProcessConfig::load
#[test]
fn test_process_config_load_returns_default_when_no_config_dir() {
    use swe_edge_configbuilder::create_loader_for_dir;
    // Point at a nonexistent dir — should return Default
    let loader = create_loader_for_dir(std::path::Path::new("/nonexistent/path/xyz_swe_test"));
    let cfg = ProcessConfig::load(&loader);
    // Absent section returns default
    assert!(cfg.is_ok());
    assert_eq!(cfg.unwrap().allow_commands.len(), 0);
}
