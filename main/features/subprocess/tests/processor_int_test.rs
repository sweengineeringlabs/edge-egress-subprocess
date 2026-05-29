//! Integration tests for the `Processor` trait.

use swe_edge_egress_subprocess::Processor;

/// @covers: Processor
#[test]
fn test_processor_is_object_safe() {
    fn _assert(_: &dyn Processor) {}
}
