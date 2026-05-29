//! Integration tests for swe-edge-egress-process.

use swe_edge_egress_subprocess::*;

/// @covers: create_swe_edge_egress_subprocess
#[test]
fn test_create_swe_edge_egress_subprocess_succeeds() {
    let svc = ProcessSvc::service();
    assert!(svc.execute().is_ok());
}
