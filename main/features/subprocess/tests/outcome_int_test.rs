//! Integration tests for SubprocessResult.

use swe_edge_egress_subprocess::SubprocessResult;

/// @covers: SubprocessResult::Completed
#[test]
fn test_subprocess_result_completed_is_debuggable() {
    let r = SubprocessResult::Completed {
        exit_code: 0,
        stdout: "ok".into(),
        stderr: String::new(),
    };
    let _ = format!("{r:?}");
}

/// @covers: SubprocessResult::Denied
#[test]
fn test_subprocess_result_denied_is_debuggable() {
    let r = SubprocessResult::Denied {
        command: "echo".into(),
    };
    let _ = format!("{r:?}");
}

/// @covers: SubprocessResult::IsolationFailed
#[test]
fn test_subprocess_result_isolation_failed_is_debuggable() {
    let r = SubprocessResult::IsolationFailed {
        profile: "noop".into(),
        reason: "test".into(),
    };
    let _ = format!("{r:?}");
}
