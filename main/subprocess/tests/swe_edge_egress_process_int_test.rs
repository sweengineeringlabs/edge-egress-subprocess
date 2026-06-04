//! Integration tests for swe-edge-egress-subprocess.

use swe_edge_egress_subprocess::*;

/// @covers: create_swe_edge_egress_subprocess
#[test]
fn test_create_swe_edge_egress_subprocess_succeeds() {
    let svc = SubprocessSvc::service();
    assert!(svc.execute().is_ok());
}
