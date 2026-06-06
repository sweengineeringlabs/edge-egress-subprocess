//! Integration tests for `SubprocessArgs`.

use swe_edge_egress_subprocess::SubprocessArgs;

/// @covers: SubprocessArgs::builder
#[test]
fn test_subprocess_args_builder_returns_builder_with_empty_argv() {
    let args = SubprocessArgs::builder().build();
    assert!(args.argv.is_empty());
}

/// @covers: SubprocessArgs::builder
#[test]
fn test_subprocess_args_builder_argv_is_set() {
    let args = SubprocessArgs::builder()
        .argv(vec!["sh".into(), "-c".into(), "true".into()])
        .build();
    assert_eq!(args.argv.len(), 3);
}
