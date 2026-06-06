//! Unit tests for swe-edge-egress-subprocess.

use swe_edge_egress_subprocess::*;

/// @covers: create_swe_edge_egress_subprocess
#[test]
fn test_create_swe_edge_egress_subprocess_returns_working_impl() {
    let svc = SubprocessSvc::service();
    assert!(svc.execute().is_ok());
}
