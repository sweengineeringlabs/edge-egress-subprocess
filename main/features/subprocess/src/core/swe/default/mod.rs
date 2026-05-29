//! Default implementations for the swe_edge_egress_subprocess service.

pub(crate) mod swe_edge_egress_process;
pub(crate) mod validator;

pub(crate) use swe_edge_egress_process::DefaultSweEdgeEgressProcess;
pub(crate) use validator::DefaultProcessValidator;
