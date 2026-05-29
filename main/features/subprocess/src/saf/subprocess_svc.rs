//! SAF — `SubprocessSvc` factory wiring for subprocess execution.

pub use crate::api::{
    AllowList, Error, IsolationError, IsolationProfile, Processor, SubprocessArgs,
    SubprocessArgsBuilder, SubprocessConfig, SubprocessConfigBuilder, SubprocessResult,
    SubprocessRunner, SubprocessRunnerExtension, SubprocessSvc, SweEdgeEgressProcess, Validator,
};
use crate::core::default::DefaultSubprocessRunner;
use crate::core::extension::ExtensionRunner;
use crate::core::swe::default::{DefaultProcessValidator, DefaultSweEdgeEgressProcess};
pub use futures::future::BoxFuture;

impl SubprocessSvc {
    /// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
    ///
    /// Call `.build_loader()` on the result, then load policy via
    /// [`SubprocessConfig::load`]. Pass the loaded config to [`SubprocessConfig::with_argv`]
    /// to produce a per-call [`SubprocessArgs`].
    pub fn config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Return a [`SubprocessRunner`] backed by `tokio::process::Command`.
    ///
    /// The returned runner is stateless and safe to share behind
    /// `Arc<dyn SubprocessRunner>` across concurrent callers.
    ///
    /// Spawn policy — allow-list, timeout, byte cap — comes from [`SubprocessConfig`]
    /// loaded via TOML. Supply only the per-call `argv` at the call site.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::{ConfigSection as _, ConfigLoaderFactory};
    /// use swe_edge_egress_subprocess::{SubprocessSvc, SubprocessConfig, SubprocessRunner};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let loader = ConfigLoaderFactory::create_loader()?;
    /// let cfg    = SubprocessConfig::load(&loader)?;
    /// let runner = SubprocessSvc::runner();
    /// let args   = cfg.with_argv(vec!["echo".into(), "hello".into()]);
    /// let result = runner.run(args).await;
    /// # Ok(())
    /// # }
    /// ```
    pub fn runner() -> impl SubprocessRunner {
        DefaultSubprocessRunner
    }

    /// Return the default [`SweEdgeEgressProcess`] implementation.
    pub fn service() -> impl SweEdgeEgressProcess {
        DefaultSweEdgeEgressProcess
    }

    /// Return the default [`Validator`] implementation.
    pub fn validator() -> impl Validator {
        DefaultProcessValidator
    }

    /// Wrap a [`SubprocessRunnerExtension`] as a [`SubprocessRunner`].
    ///
    /// Downstream consumers implement [`SubprocessRunnerExtension`] to plug in
    /// custom subprocess execution (e.g. sandbox integration, remote execution).
    /// This method is the only entry point into the extension stack — callers
    /// receive `impl SubprocessRunner` and never name the adapter type.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use futures::future::BoxFuture;
    /// use swe_edge_egress_subprocess::{
    ///     SubprocessArgs, SubprocessResult, SubprocessRunnerExtension, SubprocessSvc, SubprocessRunner,
    /// };
    ///
    /// #[derive(Debug)]
    /// struct SandboxRunner;
    /// impl SubprocessRunnerExtension for SandboxRunner {
    ///     fn run_extended(&self, args: SubprocessArgs) -> BoxFuture<'_, SubprocessResult> {
    ///         Box::pin(async move { SubprocessResult::Denied { command: "sandbox".into() } })
    ///     }
    /// }
    ///
    /// let runner = SubprocessSvc::with_runner(SandboxRunner);
    /// // runner: impl SubprocessRunner — use exactly like the default runner
    /// ```
    pub fn with_runner(ext: impl SubprocessRunnerExtension) -> impl SubprocessRunner {
        ExtensionRunner::new(ext)
    }
}
