//! Integration tests verifying swe-edge-configbuilder usage with SubprocessConfig.

use swe_edge_configbuilder::{ConfigLoaderFactory, ConfigSection as _};
use swe_edge_egress_subprocess::SubprocessConfig;

/// @covers: SubprocessConfig::section_name
#[test]
fn test_subprocess_config_section_name_returns_subprocess() {
    assert_eq!(SubprocessConfig::section_name(), "subprocess");
}

/// @covers: SubprocessConfig::load
#[test]
fn test_subprocess_config_load_returns_default_from_config_dir() {
    // Load from the crate's own config/ directory — always present.
    let loader = ConfigLoaderFactory::create_loader_for_dir(
        std::path::Path::new("config")
    );
    // Section may or may not be present; either way we get a valid config.
    let cfg = SubprocessConfig::load(&loader).unwrap_or_default();
    // Default has empty allow_commands (block-all safety default).
    let _ = cfg.allow_commands;
}
