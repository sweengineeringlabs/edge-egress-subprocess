//! Integration tests for the `AllowList` trait.

use swe_edge_egress_subprocess::AllowList;

/// @covers: AllowList
#[test]
fn test_allow_list_is_object_safe() {
    fn _assert(_: &dyn AllowList) {}
}
