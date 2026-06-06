//! [`SubprocessExtension`] — SPI hook for downstream subprocess runner extensions.

/// Marker extension point for downstream crates that provide custom subprocess execution.
///
/// Implement [`SubprocessRunnerExtension`](crate::api::traits::subprocess_runner_extension::SubprocessRunnerExtension)
/// on a zero-size struct to register a custom runner with the
/// `swe-edge-egress-subprocess` SPI surface via [`SubprocessSvc::with_runner`].
///
/// [`SubprocessSvc::with_runner`]: crate::SubprocessSvc::with_runner
#[expect(
    dead_code,
    reason = "SEA spi/ extension anchor — registered via SubprocessSvc::with_runner"
)]
pub(crate) struct SubprocessExtension;
