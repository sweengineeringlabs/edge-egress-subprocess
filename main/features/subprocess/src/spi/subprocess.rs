//! [`SubprocessExtension`] — SPI hook for downstream subprocess runner extensions.

pub(crate) use crate::api::traits::subprocess::subprocess_runner::SubprocessRunner;
pub(crate) use crate::api::traits::subprocess::subprocess_runner_extension::SubprocessRunnerExtension;
pub(crate) use crate::api::traits::swe_edge_egress_process::SweEdgeEgressProcess;
pub(crate) use crate::api::traits::validator::Validator;

/// Marker extension point for downstream crates that provide custom subprocess execution.
///
/// Implement [`SubprocessRunnerExtension`] on a zero-size struct to register
/// a custom runner with the `swe-edge-egress-subprocess` SPI surface via
/// [`SubprocessSvc::with_runner`].
///
/// [`SubprocessSvc::with_runner`]: crate::SubprocessSvc::with_runner
pub(crate) struct SubprocessExtension;
