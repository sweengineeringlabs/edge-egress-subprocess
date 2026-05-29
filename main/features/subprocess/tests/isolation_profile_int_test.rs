//! Integration tests for the `IsolationProfile` trait and `IsolationError` enum.
// @allow: no_mocks_in_integration — StubProfile is a minimal test implementation of
// IsolationProfile required to verify the trait's object-safety and default hooks.

use std::sync::Arc;

use swe_edge_egress_subprocess::{IsolationError, IsolationProfile};

// @allow: no_mocks_in_integration
#[derive(Debug)]
struct StubProfile;

// @allow: no_mocks_in_integration
impl IsolationProfile for StubProfile {
    fn name(&self) -> &str {
        "stub"
    }
}

/// @covers: IsolationProfile::configure
// @allow: no_mocks_in_integration
#[test]
fn test_isolation_profile_default_configure_returns_ok() {
    let p = StubProfile;
    let mut cmd = tokio::process::Command::new("echo");
    assert!(p.configure(&mut cmd).is_ok());
}

/// @covers: IsolationProfile
#[test]
fn test_isolation_profile_is_object_safe() {
    fn _assert(_: &dyn IsolationProfile) {}
}

/// @covers: IsolationProfile
// @allow: no_mocks_in_integration
#[test]
fn test_isolation_profile_can_be_stored_as_arc_dyn() {
    let _: Arc<dyn IsolationProfile> = Arc::new(StubProfile);
}

/// @covers: IsolationError::UnknownProfile
#[test]
fn test_isolation_error_unknown_profile_message() {
    let e = IsolationError::UnknownProfile {
        profile: "ghost".into(),
    };
    assert!(e.to_string().contains("ghost"));
}

/// @covers: IsolationError::UnsupportedPlatform
#[test]
fn test_isolation_error_unsupported_platform_message() {
    let e = IsolationError::UnsupportedPlatform {
        profile: "seccomp".into(),
    };
    assert!(e.to_string().contains("seccomp"));
}
