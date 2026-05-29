//! Integration tests for the Validator trait.

use swe_edge_egress_subprocess::*;

/// @covers: Validator
#[test]
fn test_validator_validates_successfully() {
    let v = ProcessSvc::validator();
    assert!(v.validate().is_ok());
}
