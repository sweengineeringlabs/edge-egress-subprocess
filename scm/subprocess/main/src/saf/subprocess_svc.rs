//! SAF — `SubprocessSvc` factory wiring for subprocess execution.

use crate::api::types::subprocess_svc::SubprocessSvc;
use crate::core::default::process_validator::DefaultProcessValidator;
use crate::core::default::subprocess_runner::DefaultSubprocessRunner;
use crate::core::default::swe_edge_egress_process::DefaultSweEdgeEgressProcess;
use crate::core::extension::extension_runner::ExtensionRunner;

impl SubprocessSvc {
    /// Return a [`SubprocessRunner`] backed by `tokio::process::Command`.
    ///
    /// The returned runner is stateless and safe to share behind
    /// `Arc<dyn SubprocessRunner>` across concurrent callers.
    ///
    /// Spawn policy — allow-list, timeout, byte cap — comes from [`SubprocessConfig`]
    /// loaded via TOML. Load config via [`swe_edge_configbuilder::create_loader`]
    /// and supply only the per-call `argv` at the call site.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::{ConfigLoaderFactory, ConfigSection as _};
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
    pub fn runner() -> impl crate::api::traits::subprocess::runner::SubprocessRunner {
        DefaultSubprocessRunner
    }

    /// Return the default [`SweEdgeEgressProcess`] implementation.
    pub fn service() -> impl crate::api::traits::swe_edge_egress_process::SweEdgeEgressProcess {
        DefaultSweEdgeEgressProcess
    }

    /// Return a config builder pre-seeded with this crate's package name and version.
    pub fn config_builder(
    ) -> crate::api::types::application_config_builder::ApplicationConfigBuilder {
        swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Return the default [`Validator`] implementation.
    pub fn validator() -> impl crate::api::traits::validator::Validator {
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
    pub fn with_runner(
        ext: impl crate::api::traits::subprocess::runner_extension::SubprocessRunnerExtension,
    ) -> impl crate::api::traits::subprocess::runner::SubprocessRunner {
        ExtensionRunner::new(ext)
    }
}
