//! Integration tests for `IsolationError`.

use swe_edge_egress_subprocess::IsolationError;

/// @covers: IsolationError::UnknownProfile
#[test]
fn test_isolation_error_unknown_profile_contains_name() {
    let e = IsolationError::UnknownProfile {
        profile: "sandbox".into(),
    };
    assert!(e.to_string().contains("sandbox"));
}

/// @covers: IsolationError::SeccompFailed
#[test]
fn test_isolation_error_seccomp_failed_contains_reason() {
    let e = IsolationError::SeccompFailed {
        profile: "strict".into(),
        reason: "filter load failed".into(),
    };
    let msg = e.to_string();
    assert!(msg.contains("strict"));
    assert!(msg.contains("filter load failed"));
}
