//! Integration tests for `SubprocessResult`.

use swe_edge_egress_subprocess::SubprocessResult;

/// @covers: SubprocessResult::Denied
#[test]
fn test_subprocess_result_denied_is_debug() {
    let r = SubprocessResult::Denied {
        command: "rm".into(),
    };
    assert!(format!("{r:?}").contains("Denied"));
}

/// @covers: SubprocessResult::SpawnFailed
#[test]
fn test_subprocess_result_spawn_failed_carries_reason() {
    let r = SubprocessResult::SpawnFailed {
        reason: "binary not found".into(),
    };
    let dbg = format!("{r:?}");
    assert!(dbg.contains("SpawnFailed"));
}

/// @covers: SubprocessResult::TimedOut
#[test]
fn test_subprocess_result_timed_out_carries_timeout_ms() {
    let r = SubprocessResult::TimedOut { timeout_ms: 200 };
    let dbg = format!("{r:?}");
    assert!(dbg.contains("TimedOut"));
}
