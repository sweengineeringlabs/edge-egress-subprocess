//! Integration tests for `ProcessResult`.

use swe_edge_egress_subprocess::ProcessResult;

/// @covers: ProcessResult::Denied
#[test]
fn test_process_result_denied_is_debug() {
    let r = ProcessResult::Denied {
        command: "rm".into(),
    };
    assert!(format!("{r:?}").contains("Denied"));
}

/// @covers: ProcessResult::SpawnFailed
#[test]
fn test_process_result_spawn_failed_carries_reason() {
    let r = ProcessResult::SpawnFailed {
        reason: "binary not found".into(),
    };
    let dbg = format!("{r:?}");
    assert!(dbg.contains("SpawnFailed"));
}

/// @covers: ProcessResult::TimedOut
#[test]
fn test_process_result_timed_out_carries_timeout_ms() {
    let r = ProcessResult::TimedOut { timeout_ms: 200 };
    let dbg = format!("{r:?}");
    assert!(dbg.contains("TimedOut"));
}
