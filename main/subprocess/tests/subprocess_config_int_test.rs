//! Integration tests for `SubprocessConfig`.

use std::collections::HashMap;

use swe_edge_configbuilder::ConfigSection as _;
use swe_edge_egress_subprocess::SubprocessConfig;

/// @covers: SubprocessConfig::default
#[test]
fn test_subprocess_config_default_allow_commands_empty() {
    assert!(SubprocessConfig::default().allow_commands.is_empty());
}

/// @covers: SubprocessConfig::section_name
#[test]
fn test_subprocess_config_section_name_is_subprocess() {
    assert_eq!(SubprocessConfig::section_name(), "subprocess");
}

/// @covers: with_argv
#[test]
fn test_with_argv_sets_argv_and_propagates_policy() {
    let cfg = SubprocessConfig {
        allow_commands: vec!["ffmpeg".into()],
        timeout_ms: 5_000,
        output_bytes_cap: 512,
        cwd: None,
        env: HashMap::new(),
        cpu_time_ms: None,
        memory_bytes: None,
    };
    let args = cfg.with_argv(vec!["ffmpeg".into(), "-i".into(), "in.mp4".into()]);
    assert_eq!(args.argv, vec!["ffmpeg", "-i", "in.mp4"]);
    assert_eq!(args.allow_commands, vec!["ffmpeg"]);
    assert_eq!(args.timeout_ms, Some(5_000));
    assert_eq!(args.output_bytes_cap, Some(512));
}

/// @covers: with_argv
#[test]
fn test_subprocess_config_default_cpu_and_memory_are_none() {
    let cfg = SubprocessConfig::default();
    assert!(cfg.cpu_time_ms.is_none());
    assert!(cfg.memory_bytes.is_none());
}

/// @covers: with_argv
#[test]
fn test_with_argv_propagates_cpu_and_memory_limits() {
    let cfg = SubprocessConfig {
        allow_commands: vec!["sh".into()],
        timeout_ms: 5_000,
        output_bytes_cap: 1_024,
        cwd: None,
        env: Default::default(),
        cpu_time_ms: Some(3_000),
        memory_bytes: Some(536_870_912),
    };
    let args = cfg.with_argv(vec!["sh".into()]);
    assert_eq!(args.cpu_time_ms, Some(3_000));
    assert_eq!(args.memory_bytes, Some(536_870_912));
}

/// @covers: with_argv
#[test]
fn test_with_argv_empty_allow_commands_blocks_all() {
    let cfg = SubprocessConfig::default();
    let args = cfg.with_argv(vec!["echo".into()]);
    assert!(args.allow_commands.is_empty());
}
