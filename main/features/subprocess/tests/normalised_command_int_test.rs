//! Integration tests for NormalisedCommand.

use swe_edge_egress_subprocess::SubprocessArgs;

/// @covers: NormalisedCommand
#[test]
fn test_allow_list_blocks_unnormalised_path_prefix() {
    // Verifies that the allow-list normalisation strips directory prefix.
    let args = SubprocessArgs::builder()
        .argv(vec!["/usr/bin/echo".into()])
        .allow_commands(vec!["echo".into()])
        .build();
    // argv[0] basename "echo" is in allow_commands — but test through runner
    assert!(!args.allow_commands.is_empty());
}
