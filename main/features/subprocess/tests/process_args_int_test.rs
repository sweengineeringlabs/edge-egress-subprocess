//! Integration tests for `ProcessArgs`.

use swe_edge_egress_subprocess::ProcessArgs;

/// @covers: ProcessArgs::builder
#[test]
fn test_process_args_builder_returns_builder_with_empty_argv() {
    let args = ProcessArgs::builder().build();
    assert!(args.argv.is_empty());
}

/// @covers: ProcessArgs::builder
#[test]
fn test_process_args_builder_argv_is_set() {
    let args = ProcessArgs::builder()
        .argv(vec!["sh".into(), "-c".into(), "true".into()])
        .build();
    assert_eq!(args.argv.len(), 3);
}
