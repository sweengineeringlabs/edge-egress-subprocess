//! `SweEdgeEgressProcess` — primary service trait.

use crate::api::error::error::Error;

/// Primary service trait for swe_edge_egress_subprocess.
pub trait SweEdgeEgressProcess: Send + Sync {
    /// Execute the primary operation.
    fn execute(&self) -> Result<(), Error>;
}
