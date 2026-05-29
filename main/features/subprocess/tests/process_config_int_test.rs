//! Integration tests for `SubprocessConfig`.

use swe_edge_egress_subprocess::SubprocessConfig;

/// @covers: SubprocessConfig::with_argv
#[test]
fn test_subprocess_config_with_argv_produces_correct_argv() {
    let cfg = SubprocessConfig::default();
    let args = cfg.with_argv(vec!["cat".into(), "/etc/hostname".into()]);
    assert_eq!(args.argv[0], "cat");
}

/// @covers: SubprocessConfig::default_timeout_ms
#[test]
fn test_subprocess_config_default_timeout_is_nonzero() {
    assert!(SubprocessConfig::default_timeout_ms() > 0);
}

/// @covers: SubprocessConfig::default_output_bytes_cap
#[test]
fn test_subprocess_config_default_output_bytes_cap_is_nonzero() {
    assert!(SubprocessConfig::default_output_bytes_cap() > 0);
}
