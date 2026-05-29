//! Integration tests for `ProcessConfigBuilder`.

use swe_edge_egress_subprocess::ProcessConfigBuilder;

/// @covers: ProcessConfigBuilder::build
#[test]
fn test_process_config_builder_build_returns_default_allow_commands_empty() {
    let cfg = ProcessConfigBuilder::default().build();
    assert!(cfg.allow_commands.is_empty());
}

/// @covers: ProcessConfigBuilder::allow_commands
#[test]
fn test_process_config_builder_sets_allow_commands() {
    let cfg = ProcessConfigBuilder::default()
        .allow_commands(vec!["ffmpeg".into()])
        .build();
    assert_eq!(cfg.allow_commands, vec!["ffmpeg"]);
}

/// @covers: ProcessConfigBuilder::timeout_ms
#[test]
fn test_process_config_builder_sets_timeout_ms() {
    let cfg = ProcessConfigBuilder::default().timeout_ms(5_000).build();
    assert_eq!(cfg.timeout_ms, 5_000);
}
