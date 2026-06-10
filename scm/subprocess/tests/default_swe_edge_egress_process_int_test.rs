//! DefaultSweEdgeEgressProcess integration tests for swe-edge-egress-subprocess.

use swe_edge_egress_subprocess::*;

/// @covers: DefaultSweEdgeEgressProcess
#[test]
fn test_default_swe_edge_egress_subprocess_creates_and_executes() {
    let svc = SubprocessSvc::service();
    assert!(svc.execute().is_ok());
}
