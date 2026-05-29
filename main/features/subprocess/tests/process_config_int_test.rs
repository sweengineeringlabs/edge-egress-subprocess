//! Integration tests for `ProcessConfig`.

use swe_edge_egress_subprocess::ProcessConfig;

/// @covers: ProcessConfig::with_argv
#[test]
fn test_process_config_with_argv_produces_correct_argv() {
    let cfg = ProcessConfig::default();
    let args = cfg.with_argv(vec!["cat".into(), "/etc/hostname".into()]);
    assert_eq!(args.argv[0], "cat");
}

/// @covers: ProcessConfig::default_timeout_ms
#[test]
fn test_process_config_default_timeout_is_nonzero() {
    assert!(ProcessConfig::default_timeout_ms() > 0);
}

/// @covers: ProcessConfig::default_output_bytes_cap
#[test]
fn test_process_config_default_output_bytes_cap_is_nonzero() {
    assert!(ProcessConfig::default_output_bytes_cap() > 0);
}
