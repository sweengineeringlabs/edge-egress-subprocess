//! Integration tests for SubprocessArgsBuilder.

use swe_edge_egress_subprocess::SubprocessArgs;

/// @covers: SubprocessArgsBuilder::build
#[test]
fn test_subprocess_args_builder_default_argv_is_empty() {
    let args = SubprocessArgs::builder().build();
    assert!(args.argv.is_empty());
}

/// @covers: SubprocessArgsBuilder::argv
#[test]
fn test_subprocess_args_builder_sets_argv() {
    let args = SubprocessArgs::builder().argv(vec!["echo".into()]).build();
    assert_eq!(args.argv, vec!["echo"]);
}
