//! Integration tests for swe-edge-egress-process API traits.

use swe_edge_egress_subprocess::*;

/// @covers: Validator
#[test]
fn test_validator_trait_is_object_safe() {
    fn _accept(_v: &dyn Validator) {}
}

/// @covers: create_validator
#[test]
fn test_create_validator_returns_valid_impl() {
    let v = ProcessSvc::validator();
    assert!(v.validate().is_ok());
}
