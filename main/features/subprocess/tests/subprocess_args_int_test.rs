//! Integration tests for `ProcessArgs` and `ProcessArgsBuilder`.

use swe_edge_egress_subprocess::{ProcessArgs, ProcessArgsBuilder};

/// @covers: build
#[test]
fn test_process_args_builder_defaults_to_empty_argv() {
    let args = ProcessArgs::builder().build();
    assert!(args.argv.is_empty());
}

/// @covers: build
#[test]
fn test_process_args_builder_sets_argv() {
    let args = ProcessArgs::builder()
        .argv(vec!["echo".into(), "hi".into()])
        .build();
    assert_eq!(args.argv, vec!["echo", "hi"]);
}

/// @covers: timeout_ms
#[test]
fn test_process_args_builder_timeout_none_by_default() {
    let args = ProcessArgs::builder().build();
    assert!(args.timeout_ms.is_none());
}

/// @covers: output_bytes_cap
#[test]
fn test_process_args_builder_output_bytes_cap_none_by_default() {
    let args = ProcessArgs::builder().build();
    assert!(args.output_bytes_cap.is_none());
}

/// @covers: env
#[test]
fn test_process_args_builder_env_empty_by_default() {
    let args = ProcessArgs::builder().build();
    assert!(args.env.is_empty());
}

/// @covers: allow_commands
#[test]
fn test_process_args_builder_allow_commands_empty_by_default() {
    let args = ProcessArgs::builder().build();
    assert!(args.allow_commands.is_empty());
}

/// @covers: cwd
#[test]
fn test_process_args_builder_sets_cwd() {
    let args = ProcessArgs::builder().cwd("/tmp/workdir").build();
    assert_eq!(args.cwd, Some(std::path::PathBuf::from("/tmp/workdir")));
}

/// @covers: cpu_time_ms
#[test]
fn test_process_args_builder_sets_cpu_time_ms() {
    let args = ProcessArgs::builder().cpu_time_ms(5_000).build();
    assert_eq!(args.cpu_time_ms, Some(5_000));
}

/// @covers: memory_bytes
#[test]
fn test_process_args_builder_sets_memory_bytes() {
    let args = ProcessArgs::builder().memory_bytes(1_073_741_824).build();
    assert_eq!(args.memory_bytes, Some(1_073_741_824));
}

/// @covers: isolation_profile
#[test]
fn test_process_args_builder_isolation_profile_none_by_default() {
    let args = ProcessArgs::builder().build();
    assert!(args.isolation_profile.is_none());
}

/// @covers: ProcessArgsBuilder::argv
#[test]
fn test_process_args_builder_argv_replaces_empty() {
    let _b: ProcessArgsBuilder = ProcessArgs::builder().argv(vec!["sh".into()]);
    // Just verify the builder method is callable — the built value is tested elsewhere.
}
