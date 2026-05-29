//! Integration tests for ProcessResult.

use swe_edge_egress_subprocess::ProcessResult;

/// @covers: ProcessResult::Completed
#[test]
fn test_process_result_completed_is_debuggable() {
    let r = ProcessResult::Completed {
        exit_code: 0,
        stdout: "ok".into(),
        stderr: String::new(),
    };
    let _ = format!("{r:?}");
}

/// @covers: ProcessResult::Denied
#[test]
fn test_process_result_denied_is_debuggable() {
    let r = ProcessResult::Denied {
        command: "echo".into(),
    };
    let _ = format!("{r:?}");
}

/// @covers: ProcessResult::IsolationFailed
#[test]
fn test_process_result_isolation_failed_is_debuggable() {
    let r = ProcessResult::IsolationFailed {
        profile: "noop".into(),
        reason: "test".into(),
    };
    let _ = format!("{r:?}");
}
