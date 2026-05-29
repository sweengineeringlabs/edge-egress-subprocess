//! Integration tests for `SubprocessConfigBuilder`.

use swe_edge_egress_subprocess::SubprocessConfigBuilder;

/// @covers: SubprocessConfigBuilder::build
#[test]
fn test_subprocess_config_builder_build_returns_default_allow_commands_empty() {
    let cfg = SubprocessConfigBuilder::default().build();
    assert!(cfg.allow_commands.is_empty());
}

/// @covers: SubprocessConfigBuilder::allow_commands
#[test]
fn test_subprocess_config_builder_sets_allow_commands() {
    let cfg = SubprocessConfigBuilder::default()
        .allow_commands(vec!["ffmpeg".into()])
        .build();
    assert_eq!(cfg.allow_commands, vec!["ffmpeg"]);
}

/// @covers: SubprocessConfigBuilder::timeout_ms
#[test]
fn test_subprocess_config_builder_sets_timeout_ms() {
    let cfg = SubprocessConfigBuilder::default().timeout_ms(5_000).build();
    assert_eq!(cfg.timeout_ms, 5_000);
}
