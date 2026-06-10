//! Integration tests for `SubprocessArgsBuilder`.

use swe_edge_egress_subprocess::{SubprocessArgs, SubprocessArgsBuilder};

/// @covers: SubprocessArgsBuilder::build
#[test]
fn test_subprocess_args_builder_all_optional_fields_default_to_none() {
    let args: SubprocessArgs = SubprocessArgsBuilder::default().build();
    assert!(args.timeout_ms.is_none());
    assert!(args.output_bytes_cap.is_none());
    assert!(args.cpu_time_ms.is_none());
    assert!(args.memory_bytes.is_none());
    assert!(args.isolation_profile.is_none());
}

/// @covers: SubprocessArgsBuilder::output_bytes_cap
#[test]
fn test_subprocess_args_builder_output_bytes_cap_roundtrips() {
    let args = SubprocessArgsBuilder::default()
        .output_bytes_cap(4_096)
        .build();
    assert_eq!(args.output_bytes_cap, Some(4_096));
}
