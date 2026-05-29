//! Integration tests for swe-edge-egress-process SAF facade.

use swe_edge_egress_subprocess::*;

/// @covers: create_swe_edge_egress_subprocess
#[test]
fn test_create_swe_edge_egress_subprocess_via_saf_succeeds() {
    let svc = ProcessSvc::service();
    assert!(svc.execute().is_ok());
}

/// @covers: create_validator
#[test]
fn test_create_validator_via_saf_succeeds() {
    let v = ProcessSvc::validator();
    assert!(v.validate().is_ok());
}
