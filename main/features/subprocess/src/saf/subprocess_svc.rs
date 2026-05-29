//! SAF — `ProcessSvc` factory wiring for subprocess execution.

use swe_edge_configbuilder::ConfigBuilder as _;

pub use crate::api::{
    AllowList, Error, IsolationError, IsolationProfile, ProcessArgs, ProcessArgsBuilder,
    ProcessConfig, ProcessConfigBuilder, ProcessResult, ProcessRunner, ProcessSvc, Processor,
    SweEdgeEgressProcess, Validator,
};
use crate::core::default::DefaultProcessRunner;
use crate::core::extension::ExtensionRunner;
use crate::core::swe::default::{DefaultProcessValidator, DefaultSweEdgeEgressProcess};
pub use crate::spi::ProcessRunnerExtension;
pub use futures::future::BoxFuture;

impl ProcessSvc {
    /// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
    ///
    /// Call `.build_loader()` on the result, then load policy via
    /// [`ProcessConfig::load`]. Pass the loaded config to [`ProcessConfig::with_argv`]
    /// to produce a per-call [`ProcessArgs`].
    pub fn config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
        swe_edge_configbuilder::create_config_builder()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Return a [`ProcessRunner`] backed by `tokio::process::Command`.
    ///
    /// The returned runner is stateless and safe to share behind
    /// `Arc<dyn ProcessRunner>` across concurrent callers.
    ///
    /// Spawn policy — allow-list, timeout, byte cap — comes from [`ProcessConfig`]
    /// loaded via TOML. Supply only the per-call `argv` at the call site.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::{ConfigSection as _, create_loader};
    /// use swe_edge_egress_subprocess::{ProcessSvc, ProcessConfig, ProcessRunner};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let loader = create_loader();
    /// let cfg    = ProcessConfig::load(&loader)?;
    /// let runner = ProcessSvc::runner();
    /// let args   = cfg.with_argv(vec!["echo".into(), "hello".into()]);
    /// let result = runner.run(args).await;
    /// # Ok(())
    /// # }
    /// ```
    pub fn runner() -> impl ProcessRunner {
        DefaultProcessRunner
    }

    /// Return the default [`SweEdgeEgressProcess`] implementation.
    pub fn service() -> impl SweEdgeEgressProcess {
        DefaultSweEdgeEgressProcess
    }

    /// Return the default [`Validator`] implementation.
    pub fn validator() -> impl Validator {
        DefaultProcessValidator
    }

    /// Wrap a [`ProcessRunnerExtension`] as a [`ProcessRunner`].
    ///
    /// Downstream consumers implement [`ProcessRunnerExtension`] to plug in
    /// custom subprocess execution (e.g. sandbox integration, remote execution).
    /// This method is the only entry point into the extension stack — callers
    /// receive `impl ProcessRunner` and never name the adapter type.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use futures::future::BoxFuture;
    /// use swe_edge_egress_subprocess::{
    ///     ProcessArgs, ProcessResult, ProcessRunnerExtension, ProcessSvc, ProcessRunner,
    /// };
    ///
    /// #[derive(Debug)]
    /// struct SandboxRunner;
    /// impl ProcessRunnerExtension for SandboxRunner {
    ///     fn run_extended(&self, args: ProcessArgs) -> BoxFuture<'_, ProcessResult> {
    ///         Box::pin(async move { ProcessResult::Denied { command: "sandbox".into() } })
    ///     }
    /// }
    ///
    /// let runner = ProcessSvc::with_runner(SandboxRunner);
    /// // runner: impl ProcessRunner — use exactly like the default runner
    /// ```
    pub fn with_runner(ext: impl ProcessRunnerExtension) -> impl ProcessRunner {
        ExtensionRunner::new(ext)
    }
}
