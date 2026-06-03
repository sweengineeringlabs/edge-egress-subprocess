//! SPI — service-provider interface extension hooks for downstream consumers.
//!
//! The presence of this directory signals that `saf/` factory functions
//! may return `impl Trait` for downstream polymorphism (SEA Rule 195).
//!
//! Downstream crates implementing custom subprocess runners should implement
//! [`SubprocessRunnerExtension`] and register them via [`SubprocessSvc::with_runner`].
//!
//! [`SubprocessRunnerExtension`]: crate::api::traits::subprocess::subprocess_runner_extension::SubprocessRunnerExtension
//! [`SubprocessSvc::with_runner`]: crate::SubprocessSvc::with_runner

pub(crate) mod subprocess;
