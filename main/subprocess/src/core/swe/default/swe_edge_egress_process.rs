//! Default `SweEdgeEgressProcess` implementation.

use crate::api::error::error::Error;
use crate::api::traits::swe_edge_egress_process::SweEdgeEgressProcess;

/// Default implementation of the [`SweEdgeEgressProcess`] trait.
#[derive(Debug, Default)]
pub(crate) struct DefaultSweEdgeEgressProcess;

impl SweEdgeEgressProcess for DefaultSweEdgeEgressProcess {
    fn execute(&self) -> Result<(), Error> {
        tracing::debug!("executing swe_edge_egress_subprocess");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_constructs_swe_edge_egress_subprocess() {
        let _svc = DefaultSweEdgeEgressProcess;
    }

    #[test]
    fn test_execute_succeeds() {
        let svc = DefaultSweEdgeEgressProcess;
        assert!(svc.execute().is_ok());
    }
}
