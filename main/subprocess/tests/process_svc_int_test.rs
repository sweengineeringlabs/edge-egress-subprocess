//! Integration tests for swe-edge-egress-subprocess SAF facade.

use swe_edge_egress_subprocess::*;

/// @covers: SubprocessSvc::service
#[test]
fn test_create_swe_edge_egress_subprocess_via_saf_succeeds() {
    let svc = SubprocessSvc::service();
    assert!(svc.execute().is_ok());
}

/// @covers: SubprocessSvc::validator
#[test]
fn test_create_validator_via_saf_succeeds() {
    let v = SubprocessSvc::validator();
    assert!(v.validate().is_ok());
}
