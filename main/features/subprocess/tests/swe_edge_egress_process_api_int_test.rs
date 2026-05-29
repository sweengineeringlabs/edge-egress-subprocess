//! API trait integration tests for swe-edge-egress-process.

use swe_edge_egress_subprocess::*;

/// @covers: SweEdgeEgressProcess
#[test]
fn test_swe_edge_egress_subprocess_trait_is_object_safe() {
    fn _accept(_s: &dyn SweEdgeEgressProcess) {}
}
